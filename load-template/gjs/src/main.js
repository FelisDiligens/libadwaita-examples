import Adw from 'gi://Adw?version=1';
import Window from './window.js';

const APP_ID = "com.example.AdwApplication";

// Create a new application
let app = new Adw.Application({
    application_id: APP_ID
});

// Connect to "activate" signal of `app`
app.connect("activate", () => {
    // Create a window and set the title
    let window = new Window({
        application: app
    });
    window.set_title("Hello World in GJS");
    // Present window
    window.present();
});

// Run the application
app.run([]);