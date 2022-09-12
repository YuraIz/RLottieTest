use glib::clone;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib};
use adw::subclass::prelude::*;

use crate::config::VERSION;
use crate::RlottietestWindow;

mod imp {
    use super::*;

    #[derive(Debug, Default)]
    pub struct RlottietestApplication {}

    #[glib::object_subclass]
    impl ObjectSubclass for RlottietestApplication {
        const NAME: &'static str = "RlottietestApplication";
        type Type = super::RlottietestApplication;
        type ParentType = adw::Application;
    }

    impl ObjectImpl for RlottietestApplication {
        fn constructed(&self, obj: &Self::Type) {
            self.parent_constructed(obj);

            obj.setup_gactions();
            obj.set_accels_for_action("app.quit", &["<primary>q"]);
        }
    }

    impl ApplicationImpl for RlottietestApplication {
        // We connect to the activate callback to create a window when the application
        // has been launched. Additionally, this callback notifies us when the user
        // tries to launch a "second instance" of the application. When they try
        // to do that, we'll just present any existing window.
        fn activate(&self, application: &Self::Type) {
            // Get the current window or create one if necessary
            let window = if let Some(window) = application.active_window() {
                window
            } else {
                let window = RlottietestWindow::new(application);
                window.upcast()
            };

            // Ask the window manager/compositor to present the window
            window.present();
        }
    }

    impl GtkApplicationImpl for RlottietestApplication {}
    impl AdwApplicationImpl for RlottietestApplication {}
}

glib::wrapper! {
    pub struct RlottietestApplication(ObjectSubclass<imp::RlottietestApplication>)
        @extends gio::Application, gtk::Application, adw::Application,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl RlottietestApplication {
    pub fn new(application_id: &str, flags: &gio::ApplicationFlags) -> Self {
        glib::Object::new(&[("application-id", &application_id), ("flags", flags)])
            .expect("Failed to create RlottietestApplication")
    }

    fn setup_gactions(&self) {
        let quit_action = gio::SimpleAction::new("quit", None);
        quit_action.connect_activate(clone!(@weak self as app => move |_, _| {
            app.quit();
        }));
        self.add_action(&quit_action);

        let about_action = gio::SimpleAction::new("about", None);
        about_action.connect_activate(clone!(@weak self as app => move |_, _| {
            app.show_about();
        }));
        self.add_action(&about_action);
    }

    fn show_about(&self) {
        let window = self.active_window().unwrap();
        let dialog = gtk::AboutDialog::builder()
            .transient_for(&window)
            .modal(true)
            .program_name("rlottietest")
            .version(VERSION)
            .authors(vec!["YuraIz".into()])
            .build();

        dialog.present();
    }
}
