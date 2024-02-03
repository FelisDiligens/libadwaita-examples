mod imp {
    use adw::subclass::application_window::AdwApplicationWindowImpl;
    use adw::subclass::prelude::*;
    use gtk::glib::subclass::InitializingObject;
    use gtk::prelude::*;
    use gtk::subclass::application_window::ApplicationWindowImpl;
    use gtk::subclass::widget::{
        CompositeTemplateClass, CompositeTemplateInitializingExt, WidgetImpl,
    };
    use gtk::subclass::window::WindowImpl;
    use gtk::{glib, Button, CompositeTemplate};

    // Object holding the state
    #[derive(CompositeTemplate, Default)]
    #[template(resource = "/org/gtk/Example/window.ui")]
    pub struct Window {
        // referring to a GtkButton with the `id` "button"
        #[template_child]
        pub button: TemplateChild<Button>,
    }

    // The central trait for subclassing a GObject
    #[glib::object_subclass]
    impl ObjectSubclass for Window {
        // `NAME` needs to match `class` attribute of the template tag
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

    // Trait implementations:

    impl ObjectImpl for Window { // for GObject
        fn constructed(&self) {
            // Call "constructed" on parent
            self.parent_constructed();
    
            // Connect to "clicked" signal of `button`
            self.button.connect_clicked(move |button| {
                // Set the label to "Hello World!" after the button has been clicked on
                button.set_label("Hello World!");
            });
        }
    }

    impl WidgetImpl for Window {} // for GtkWidget
    impl WindowImpl for Window {} // for GtkWindow
    impl ApplicationWindowImpl for Window {} // for GtkApplicationWindow
    impl AdwApplicationWindowImpl for Window {} // for AdwApplicationWindow
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
        Object::builder()
            .property("application", app)
            .build()
    }
}
