mod imp {
    use adw::subclass::application_window::AdwApplicationWindowImpl;
    use gtk::glib::subclass::{prelude::*, InitializingObject};
    use gtk::subclass::application_window::ApplicationWindowImpl;
    use gtk::subclass::widget::{
        CompositeTemplateClass, CompositeTemplateInitializingExt, WidgetImpl,
    };
    use gtk::subclass::window::WindowImpl;
    use gtk::{glib, CompositeTemplate};

    #[derive(CompositeTemplate, Default)]
    #[template(resource = "/org/gtk/Example/window.ui")]
    pub struct Window {}

    #[glib::object_subclass]
    impl ObjectSubclass for Window {
        const NAME: &'static str = "Window";
        type Type = super::Window;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for Window {} // GObject implementation
    impl WidgetImpl for Window {} // GtkWidget implementation
    impl WindowImpl for Window {} // GtkWindow implementation
    impl ApplicationWindowImpl for Window {} // GtkApplicationWindow implementation
    impl AdwApplicationWindowImpl for Window {} // AdwApplicationWindow implementation
}

use glib::Object;
use gtk::{gio, glib};

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends adw::ApplicationWindow, gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Window {
    pub fn new(app: &adw::Application) -> Self {
        Object::builder().property("application", app).build()
    }
}
