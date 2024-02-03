from gi.repository import Gtk, Adw

@Gtk.Template(resource_path="/org/gtk/Example/window.ui")
class Window(Adw.ApplicationWindow):
    __gtype_name__ = "Window"

    def __init__(self, **kwargs):
        super().__init__(**kwargs)