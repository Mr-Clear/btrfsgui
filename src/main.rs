extern crate gtk;
extern crate gio;

use gtk::prelude::*;
use gio::prelude::*;
use std::env;
use gtk::{Application, ApplicationWindow, Button, Builder, Window};
use std::process::Command;
use gio::ApplicationExt;
use gio::prelude::ApplicationExtManual;

fn main() {
    let application = Application::new(
        Some("com.github.gtk-rs.examples.basic"),
        Default::default(),
    ).expect("failed to initialize GTK application");

    application.connect_activate(|app| {
        let glade_src = include_str!("main_window.glade");
        let builder = Builder::from_string(glade_src);
        let window: Window = builder.get_object("main_window").expect("Couldn't get window");
        window.set_application(Some(app));

        window.show_all();
    });

    application.run(&[]);

    let output = Command::new("ls")
        .arg("-halF")
        .arg(env::var("HOME").unwrap_or(".".to_string()))
        .output()
        .expect("failed to execute process");
    println!("{}", String::from_utf8_lossy(&output.stdout));
}
