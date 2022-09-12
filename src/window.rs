use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gdk, gio, glib, CompositeTemplate};

use rlottie::{Animation, Surface};

use super::lottie_animation::LottieAnimation;

mod imp {

    use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/org/example/App/window.ui")]
    pub struct RlottietestWindow {
        // Template widgets
        #[template_child]
        pub header_bar: TemplateChild<gtk::HeaderBar>,
        #[template_child]
        pub label: TemplateChild<gtk::Label>,
        #[template_child]
        pub gtk_box: TemplateChild<gtk::Box>,
        #[template_child]
        pub picture: TemplateChild<gtk::Picture>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for RlottietestWindow {
        const NAME: &'static str = "RlottietestWindow";
        type Type = super::RlottietestWindow;
        type ParentType = gtk::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for RlottietestWindow {
        fn constructed(&self, obj: &Self::Type) {
            self.parent_constructed(obj);

            let lottie_animation = LottieAnimation::new();

            lottie_animation.play();

            obj.imp().picture.set_paintable(Some(&lottie_animation));
        }
    }
    impl WidgetImpl for RlottietestWindow {}
    impl WindowImpl for RlottietestWindow {}
    impl ApplicationWindowImpl for RlottietestWindow {}
}

glib::wrapper! {
    pub struct RlottietestWindow(ObjectSubclass<imp::RlottietestWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl RlottietestWindow {
    pub fn new<P: glib::IsA<gtk::Application>>(application: &P) -> Self {
        glib::Object::new(&[("application", application)])
            .expect("Failed to create RlottietestWindow")
    }
}
