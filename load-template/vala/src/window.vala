[GtkTemplate (ui = "/org/gtk/Example/window.ui")]
public class Window : Adw.ApplicationWindow {
    // referring to a GtkButton with the `id` "button"
    [GtkChild]
    private Gtk.Button button;

    public Window (App app) {
        // Call parent constructor
        Object (application: app);
        // Connect to "clicked" signal of `button`
        button.clicked.connect (() => {
            // Set the label to "Hello World!" after the button has been clicked on
            button.set_label("Hello World!");
        });
    }
}