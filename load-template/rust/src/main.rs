mod window;

use gtk::prelude::*;
use gtk::{gio, glib};
use window::Window;

const APP_ID: &str = "com.example.AdwApplication";

fn main() -> glib::ExitCode {
    gio::resources_register_include!("gresources.gresource")
        .expect("Failed to register resources.");

    let app = adw::Application::builder().application_id(APP_ID).build();

    app.connect_activate(|app| {
        let window = Window::new(app);
        window.present();
    });

    app.run()
}