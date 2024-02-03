import Adw from 'gi://Adw?version=1';
import GObject from 'gi://GObject';
import Gio from 'gi://Gio';

const Window = GObject.registerClass({
    GTypeName: 'Window', // `GTypeName` needs to match `class` attribute of the template tag
    // Load from file using relative path (I couldn't find a way to use "resource:///org/gtk/Example/window.ui"):
    Template: Gio.File.new_for_path("./src/window.ui").get_uri(),
    Children: [
        'button' // referring to a GtkButton with the `id` "button"
    ],
}, class extends Adw.ApplicationWindow {
    constructor(params = {}) {
        // Call parent constructor
		super(params);
        // Connect to "clicked" signal of `button`
        this.button.connect("clicked", (button) => {
            // Set the label to "Hello World!" after the button has been clicked on
            this.button.set_label("Hello World!");
        });
	}
});

export default Window;