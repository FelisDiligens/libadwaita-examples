mod window;

use gtk::prelude::*;
use gtk::{gio, glib};
use window::Window;

const APP_ID: &str = "com.example.AdwApplication";

fn main() -> glib::ExitCode {
    // Register and include resources
    gio::resources_register_include!("gresources.gresource")
        .expect("Failed to register resources.");

    // Create a new application
    let app = adw::Application::builder()
        .application_id(APP_ID)
        .build();

    // Connect to "activate" signal of `app`
    app.connect_activate(|app| {
        // Create a window and set the title
        let window = Window::new(app);
        window.set_title(Some("Hello World in Rust"));
        // Present window
        window.present();
    });

    // Run the application
    app.run()
}
