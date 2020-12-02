extern crate gtk;
extern crate gio;

use gtk::prelude::*;
use std::env;
use gtk::*;
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
        let screen = window.get_screen().unwrap();
        let text: TextView = builder.get_object("test_text").expect("Couldn't get test_text");

        let style: CssProvider = CssProvider::new();
        let css = include_str!("style.css");
        style.load_from_data(&css.as_bytes()).expect("Load style failed");
        StyleContext::add_provider_for_screen(&screen, &style, STYLE_PROVIDER_PRIORITY_USER);

        window.set_application(Some(app));

        let output = Command::new("ls")
            .arg("-halF")
            .arg(env::var("HOME").unwrap_or(".".to_string()))
            .output()
            .expect("failed to execute process");
        text.get_buffer()
            .expect("Couldn't get window")
            .set_text(&String::from_utf8_lossy(&output.stdout));
        window.show_all();
    });

    application.run(&[]);
}
