from gi.repository import Gtk, Adw

@Gtk.Template(resource_path="/org/gtk/Example/window.ui")
class Window(Adw.ApplicationWindow):
    __gtype_name__ = "Window"
    button = Gtk.Template.Child()

    def __init__(self, **kwargs):
        # Call __init__ on parent:
        super().__init__(**kwargs)
        # Connect to "clicked" signal of `button`
        self.button.connect("clicked", self.on_button_clicked)

    def on_button_clicked(self, button):
        # Set the label to "Hello World!" after the button has been clicked on:
        button.set_label("Hello World!")