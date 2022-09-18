use adw::prelude::*;
use glib::clone;
use gtk::subclass::prelude::*;
use gtk::{gdk, gio, glib, CompositeTemplate};

use super::rlt;

mod imp {

    use adw::subclass::prelude::AdwApplicationWindowImpl;

    use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/org/example/App/window.ui")]
    pub struct RlottietestWindow {
        // Template widgets
        // #[template_child]
        // pub header_bar: TemplateChild<gtk::HeaderBar>,
        #[template_child]
        pub bin: TemplateChild<adw::Bin>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for RlottietestWindow {
        const NAME: &'static str = "RlottietestWindow";
        type Type = super::RlottietestWindow;
        type ParentType = adw::ApplicationWindow;

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

            let target = gtk::DropTarget::new(gdk::FileList::static_type(), gdk::DragAction::COPY);

            target.connect_drop(clone!(@weak obj => @default-return false, move
                | _, value, _, _ | {
                    if let Ok(file_list) = value.get::<gdk::FileList>() {
                        let file = file_list.files()[0].to_owned();

                        let animation = rlt::Animation::from_file(file);

                        animation.play();
                        animation.set_loop(true);
                        animation.set_halign(gtk::Align::Center);
                        animation.set_use_cache(true);

                        obj.imp().bin.set_child(Some(&animation));
                        true
                    } else {
                        false
                    }
                }
            ));

            let animation =
                rlt::Animation::from_filename("./data/animations/AuthorizationStateWaitCode.tgs");

            obj.add_controller(&target);

            animation.play();
            animation.set_loop(true);
            animation.set_halign(gtk::Align::Center);
            animation.set_use_cache(true);

            self.bin.set_child(Some(&animation));
        }
    }
    impl WidgetImpl for RlottietestWindow {}
    impl WindowImpl for RlottietestWindow {}
    impl ApplicationWindowImpl for RlottietestWindow {}
    impl AdwApplicationWindowImpl for RlottietestWindow {}
}

glib::wrapper! {
    pub struct RlottietestWindow(ObjectSubclass<imp::RlottietestWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::Window,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl RlottietestWindow {
    pub fn new<P: glib::IsA<gtk::Application>>(application: &P) -> Self {
        glib::Object::new(&[("application", application)])
            .expect("Failed to create RlottietestWindow")
    }
}
