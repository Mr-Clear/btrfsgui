mod btrfs_tools;
mod tools;

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
use glib::timeout_add_local;
use std::sync::{Arc, Mutex};
use crate::btrfs_tools::QuotaStatus;
use crate::tools::ResultExt;

fn main() {
    env::set_var("LC_ALL", "en_US.UTF-8");
    let application = Application::new(
        Some("com.github.Mr-Clear.btrfsgui"),
        Default::default(),
    ).expect("failed to initialize GTK application");

    let handlers: Arc<Mutex<Vec<DriveHandler>>> = Arc::new(Mutex::new(Vec::new()));
    let handlers_clone = handlers.clone();
    application.connect_activate(move |app| {
        let main_window_glade_src = include_str!("main_window.glade");
        let builder = Builder::from_string(main_window_glade_src);
        let window: Window = builder.get_object("main_window").expect("Couldn't get window");
        window.set_position(WindowPosition::CenterAlways);
        let screen = window.get_screen().unwrap();
        let drives_notebook: Notebook = builder.get_object("drives").expect("Couldn't get drives Notebook");

        let style: CssProvider = CssProvider::new();
        let css = include_str!("style.css");
        style.load_from_data(&css.as_bytes()).expect("Load style failed");
        StyleContext::add_provider_for_screen(&screen, &style, STYLE_PROVIDER_PRIORITY_USER);

        window.set_application(Some(app));

        let drives = BtrfsDrive::get_btrfs_drives();
        for drive in drives {
            handlers_clone.lock().unwrap().push(DriveHandler::new(drive.as_str(), &drives_notebook));
        }

        window.show_all();
    });

    glib::MainContext::default().acquire();
    let handlers_clone = handlers.clone();
    timeout_add_local(1000,  move || {
        for drive in handlers_clone.lock().unwrap().iter_mut() {
            drive.update();
        }
        return Continue(true);
    });

    application.run(&[]);
}

pub struct DriveHandler {
    pub drive: BtrfsDrive,
    pub widget: Box,
    pub path_text: Entry,
    pub device: Entry,
    pub size: Entry,
    pub used: Entry,
    pub free: Entry,
    pub quota_status: Label,
    pub quota_switch: Switch,
}

impl DriveHandler {
    fn new(path: &str, parent: &Notebook) -> DriveHandler {
        println!("{}", path);
        let drive_widget_glade_src = include_str!("drive_widget.glade");
        let builder = Builder::from_string(drive_widget_glade_src);
        let lbl = Label::new(Some(path));
        let bin: Box = builder.get_object("drive_box").expect("Couldn't get drive_box");
        parent.append_page(&bin, Some(&lbl));

        let mut d = DriveHandler { drive: BtrfsDrive::new(path),
            widget: bin,
            path_text: builder.get_object("path_txt").expect("Couldn't get path_txt"),
            device: builder.get_object("device_txt").expect("Couldn't get device_txt"),
            size: builder.get_object("size_txt").expect("Couldn't get size_txt"),
            used: builder.get_object("used_txt").expect("Couldn't get used_txt"),
            free: builder.get_object("free_txt").expect("Couldn't get free_txt"),
            quota_status: builder.get_object("quota_status_lbl").expect("Couldn't get quota_status_lbl"),
            quota_switch: builder.get_object("quota_switch").expect("Couldn't get quota_switch")};
        d.path_text.set_text(path);
        d.update();
        return d;
    }

    fn update(&mut self) {
        self.drive.update();
        self.device.set_text(self.drive.device.as_str());
        self.size.set_text(self.drive.size.file_size(options::BINARY).unwrap().as_str());
        self.used.set_text(self.drive.used.file_size(options::BINARY).unwrap().as_str());
        self.free.set_text(self.drive.free.file_size(options::BINARY).unwrap().as_str());
        let quota_status_string = match &self.drive.quota_status {
            Ok(QuotaStatus::On) => "Quota is enabled",
            Ok(QuotaStatus::Off) => "Quota is off",
            Ok(QuotaStatus::Scanning) => "Scanning...",
            Err(e) => e.as_str(),
        };
        self.quota_status.set_text(quota_status_string);
        self.quota_switch.set_state(self.drive.quota_status.contains2(&QuotaStatus::On));
    }
}
