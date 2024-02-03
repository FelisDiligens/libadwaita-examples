import gi
import sys

# Specify the version we need before importing Gtk and Adw
gi.require_version("Gtk", "4.0")
gi.require_version("Adw", "1")

from gi.repository import Gio, Adw

# Load and register resources before loading template (in window.py)
Gio.Resource._register(Gio.Resource.load("gresources.gresource"))

# Import our window
from window import Window

APP_ID = "com.example.AdwApplication"


class App(Adw.Application):
    def __init__(self, **kwargs):
        super().__init__(**kwargs)
        # Connect to "activate" signal of `app`
        self.connect("activate", self.on_activate)

    def on_activate(self, app):
        # Create a window and set the title
        self.window = Window(application=app)
        self.window.set_title("Hello World in Python")
        # Present window
        self.window.present()


if __name__ == "__main__":
    # Create a new application
    app = App(application_id=APP_ID)
    # Run the application
    app.run(sys.argv)
