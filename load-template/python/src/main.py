import gi
import sys

gi.require_version("Gtk", "4.0")
gi.require_version("Adw", "1")

from gi.repository import Gio, Adw

Gio.Resource._register(Gio.Resource.load("gresources.gresource"))

from window import Window

APP_ID = "com.example.AdwApplication"


class App(Adw.Application):
    def __init__(self, **kwargs):
        super().__init__(**kwargs)
        self.connect("activate", self.on_activate)

    def on_activate(self, app):
        self.win = Window(application=app)
        self.win.present()


if __name__ == "__main__":
    app = App(application_id=APP_ID)
    app.run(sys.argv)
