mod btrfs_tools;

extern crate gtk;
extern crate gio;

use btrfs_tools::BtrfsDrive;

use std::env;
use gio::ApplicationExt;
use gio::prelude::ApplicationExtManual;
use gtk::prelude::*;
use gtk::*;

extern crate humansize;
use humansize::{FileSize, file_size_opts as options};

fn main() {
    env::set_var("LC_ALL", "en_US.UTF-8");
    let application = Application::new(
        Some("com.github.Mr-Clear.btrfsgui"),
        Default::default(),
    ).expect("failed to initialize GTK application");

    application.connect_activate(|app| {
        let main_window_glade_src = include_str!("main_window.glade");
        let drive_widget_glade_src = include_str!("drive_widget.glade");
        let builder = Builder::from_string(main_window_glade_src);
        let window: Window = builder.get_object("main_window").expect("Couldn't get window");
        let screen = window.get_screen().unwrap();
        let drives_notebook: Notebook = builder.get_object("drives").expect("Couldn't get drives Notebook");

        let style: CssProvider = CssProvider::new();
        let css = include_str!("style.css");
        style.load_from_data(&css.as_bytes()).expect("Load style failed");
        StyleContext::add_provider_for_screen(&screen, &style, STYLE_PROVIDER_PRIORITY_USER);

        window.set_application(Some(app));

        let drives = BtrfsDrive::get_btrfs_drives();
        for drive in drives {
            println!("{}", drive.path);
            let builder = Builder::from_string(drive_widget_glade_src);
            let lbl = Label::new(Some(drive.path.as_str()));
            let bin: Box = builder.get_object("drive_box").expect("Couldn't get drive_box");
            drives_notebook.append_page(&bin, Some(&lbl));
            let path: Entry = builder.get_object("path_txt").expect("Couldn't get path_txt");
            path.set_text(drive.path.as_str());
            let device: Entry = builder.get_object("device_txt").expect("Couldn't get device_txt");
            device.set_text(drive.device.as_str());
            let size: Entry = builder.get_object("size_txt").expect("Couldn't get size_txt");
            size.set_text(drive.size.file_size(options::BINARY).unwrap().as_str());
            let used: Entry = builder.get_object("used_txt").expect("Couldn't get used_txt");
            used.set_text(drive.used.file_size(options::BINARY).unwrap().as_str());
            let free: Entry = builder.get_object("free_txt").expect("Couldn't get free_txt");
            free.set_text(drive.free.file_size(options::BINARY).unwrap().as_str());

        }

        window.show_all();
    });

    application.run(&[]);
}

