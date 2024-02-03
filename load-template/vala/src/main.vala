public class App : Adw.Application {
    public App () {
        Object (
            application_id: "com.example.AdwApplication",
            flags: ApplicationFlags.DEFAULT_FLAGS
        );
    }

    public override void activate () {
        // Create a window and set the title
        var window = new Window (this);
        window.set_title("Hello World in Vala");
        // Present window
        window.present ();
    }

    public static int main (string[] args) {
        // Create a new application
        var app = new App ();
        // Run the application
        return app.run (args);
    }
}
