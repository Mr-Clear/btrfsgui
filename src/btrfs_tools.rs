use std::process::Command;
use std::path::Path;

pub struct BtrfsDrive {
    pub path: String,
    pub device: String,
    pub size: u64,
    pub used: u64,
    pub free: u64,
    pub percentage: u8,
}

impl BtrfsDrive {
    fn new(path: &str) -> BtrfsDrive {

        let mut d = BtrfsDrive { path: path.to_string(), device: "".to_string(), size: 0, used: 0, free: 0, percentage: 0 };
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
    }

    //noinspection RsSelfConvention
    pub fn get_btrfs_drives() -> Vec<BtrfsDrive> {
        let output = Command::new("mount")
            .output().expect("failed to execute 'mount'");
        let text = std::str::from_utf8(&output.stdout).expect("Output of mount is no valid utf-8");
        let lines = text.split("\n");
        let mut drives: Vec<BtrfsDrive> = Vec::new();
        for line in lines {
            let fields: Vec<&str> = line.split(" ").collect();
            if fields.len() > 4 && fields[4] == "btrfs" {
                let drive = fields[2];
                if Path::new(drive).is_dir() {
                    drives.push(BtrfsDrive::new(drive));
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
}
