use std::process::Command;
use std::path::Path;
use regex::Regex;

#[derive(PartialEq)]
pub enum QuotaStatus {
    On,
    Off,
    Scanning,
}

pub struct BtrfsDrive {
    pub path: String,
    pub device: String,
    pub size: u64,
    pub used: u64,
    pub free: u64,
    pub percentage: u8,
    pub quota_status: Result<QuotaStatus, String>,
    pub subvolumes: Vec<BtrfsSubvolume>,
}

pub struct BtrfsSubvolume {
    pub id: u32,
    pub generation: u64,
    pub parent: u32,
    pub uuid: String,
    pub path: String,
}

impl BtrfsDrive {
    pub fn new(path: &str) -> BtrfsDrive {

        let mut d = BtrfsDrive { path: path.to_string(), device: "".to_string(), size: 0, used: 0,
            free: 0, percentage: 0 , quota_status: Err("Not initialized".to_string()), subvolumes: Vec::new()};
        d.update();
        return d;
    }

    pub fn update(&mut self) {
        let df = BtrfsDrive::df(&self.path);
        self.device = df.0;
        self.size = df.1;
        self.used = df.2;
        self.free = df.3;
        self.percentage = df.4;
        self.quota_status = self.test_quota();
        self.subvolumes = self.list_subvolumes().unwrap();
    }

    //noinspection RsSelfConvention
    pub fn get_btrfs_drives() -> Vec<String> {
        let output = Command::new("mount")
            .output().expect("failed to execute 'mount'");
        let text = std::str::from_utf8(&output.stdout).expect("Output of mount is no valid utf-8");
        let lines = text.split("\n");
        let mut drives: Vec<String> = Vec::new();
        for line in lines {
            let fields: Vec<&str> = line.split(" ").collect();
            if fields.len() > 4 && fields[4] == "btrfs" {
                let drive = fields[2];
                if Path::new(drive).is_dir() {
                    drives.push(drive.to_string());
                }
                else {
                    eprintln!("Mount is no valid directory: {}", drive);
                }
            }
        }
        return drives;
    }

    fn df(path : &String) -> (String, u64, u64, u64, u8, String) {
        let output = Command::new("df")
            .arg("-B1")
            .arg(path)
            .output().expect("failed to execute 'mount'");
        let text = std::str::from_utf8(&output.stdout).expect("Output of mount is no valid utf-8");
        let lines: Vec<&str> = text.split("\n").collect();
        let fields: Vec<&str> = lines[1].split(" ").filter(|&f| !f.is_empty()).collect();
        return (fields[0].to_string(), fields[1].parse().unwrap(), fields[2].parse().unwrap(),
                fields[3].parse().unwrap(), fields[4][..fields[4].len() - 1].parse().unwrap(), fields[5].to_string())
    }

    fn test_quota(&self) -> Result<QuotaStatus, String> {
        let output = Command::new("sudo")
            .arg("btrfs")
            .arg("qgroup")
            .arg("show")
            .arg(self.path.as_str())
            .output().expect("failed to execute 'sudo btrfs qgroup show'");

        if output.status.success() {
            return Ok(QuotaStatus::On);
        }
        let err = String::from_utf8(output.stderr).expect("Failed to decode error output.");
        if err.contains("quotas not enabled") {
            return Ok(QuotaStatus::Off);
        }
        if err.contains("data inconsistent") || err.contains("rescan is running") {
            return Ok(QuotaStatus::Scanning);
        }

        println!("{}", err);
        return Err(err);
    }

    fn list_subvolumes(&self) -> Result<Vec<BtrfsSubvolume>, String> {
        let output = Command::new("sudo")
            .arg("btrfs")
            .arg("sub")
            .arg("list")
            .arg("-pug")
            .arg(self.path.as_str())
            .output().expect("sudo btrfs sub list -pug'");

        if !output.status.success() {
            let err = String::from_utf8(output.stderr).expect("Failed to decode error output.");
            return Err(err);
        }
        let out = String::from_utf8(output.stdout).expect("Failed to decode standard output.");
        let lines = out.split("\n");
        let mut subs: Vec<BtrfsSubvolume> = Vec::new();
        let re = Regex::new(r"ID (\d+) gen (\d+) parent (\d+) top level (\d+) uuid ([0-9a-f\-]+) path (.+)")
            .expect("Failed to parse regex.");
        for line in lines {
            if line.is_empty() {
                continue;
            }
            let cap = re.captures(line).expect(format!("Failed to parse line: '{}'", line).as_str());
            subs.push( BtrfsSubvolume {
                id: cap[1].parse::<u32>().expect("Failed to parse subvolume id."),
                generation: cap[2].parse::<u64>().expect("Failed to parse subvolume generation."),
                parent: cap[3].parse::<u32>().expect("Failed to parse subvolume parent."),
                uuid: cap[5].to_string(),
                path: cap[6].to_string()});
        }

        return Ok(subs);
    }
}
