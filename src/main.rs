extern crate gtk;
extern crate gio;

use std::env;
use gio::ApplicationExt;
use gio::prelude::ApplicationExtManual;
use gtk::prelude::*;
use gtk::*;
use std::path::Path;
use std::process::Command;

fn main() {
    env::set_var("LC_ALL", "en_US.UTF-8");
    let application = Application::new(
        Some("com.github.Mr-Clear.btrfsgui"),
        Default::default(),
    ).expect("failed to initialize GTK application");

    application.connect_activate(|app| {
        let glade_src = include_str!("main_window.glade");
        let builder = Builder::from_string(glade_src);
        let window: Window = builder.get_object("main_window").expect("Couldn't get window");
        let screen = window.get_screen().unwrap();
        let drives_notebook: Notebook = builder.get_object("drives").expect("Couldn't get drives Notebook");


        let style: CssProvider = CssProvider::new();
        let css = include_str!("style.css");
        style.load_from_data(&css.as_bytes()).expect("Load style failed");
        StyleContext::add_provider_for_screen(&screen, &style, STYLE_PROVIDER_PRIORITY_USER);

        window.set_application(Some(app));

        let drives = get_btrfs_drives();
        for drive in drives {
            println!("{}", drive);
            let l1 = Label::new(Some(drive.as_str()));
            let l2 = Label::new(Some(drive.as_str()));
            drives_notebook.append_page(&l1, Some(&l2));
        }

        window.show_all();
    });

    application.run(&[]);
}

fn get_btrfs_drives() -> Vec<String> {
    let output = Command::new("mount")
        .output().expect("failed to execute 'mount'");
    let text = std::str::from_utf8(&output.stdout).expect("Output of mount is no valid utf-8");
    let lines = text.split("\n");
    let mut drives: Vec<&str> = Vec::new();
    for line in lines {
        let fields: Vec<&str> = line.split(" ").collect();
        if fields.len() > 4 && fields[4] == "btrfs" {
            let drive = fields[2];
            if Path::new(drive).is_dir() {
                drives.push(drive);
            }
            else {
                eprintln!("Mount is no valid directory: {}", drive);
            }
        }
    }
    return drives.iter().map(|s| s.to_string()).collect();
}