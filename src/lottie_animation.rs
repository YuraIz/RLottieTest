use glib::clone;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gdk, gio, glib, gsk};

use rlottie;

mod imp {
    use gtk::glib::once_cell::unsync::OnceCell;

    use super::*;
    use std::cell::{Cell, RefCell};

    #[derive(Default)]
    pub struct LottieAnimation {
        pub frame_num: Cell<usize>,
        pub animation: RefCell<Option<rlottie::Animation>>,
        pub surface: RefCell<Option<rlottie::Surface>>,
        pub intrinsic: Cell<(i32, i32, f64)>,
        pub playing: Cell<bool>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for LottieAnimation {
        const NAME: &'static str = "ContentLottieAnimation";
        type Type = super::LottieAnimation;
        type ParentType = gtk::MediaFile;
        type Interfaces = (gdk::Paintable,);
    }

    impl ObjectImpl for LottieAnimation {
        fn constructed(&self, obj: &Self::Type) {
            let mut animation =
                rlottie::Animation::from_file("./data/animations/delivery.json").unwrap();
            let size = animation.size();
            let totalframe = animation.totalframe();
            _ = self.animation.replace(Some(animation));

            let (width, height) = (size.width as i32, size.height as i32);
            let aspect_ratio = width as f64 / height as f64;

            self.intrinsic.set((width, height, aspect_ratio));

            let mut surface = rlottie::Surface::new(size);
            _ = self.surface.replace(Some(surface));

            let (sender, receiver) = glib::MainContext::sync_channel::<()>(Default::default(), 5);
            receiver.attach(
                None,
                clone!(@weak obj => @default-return glib::Continue(false), move |file| {
                        if obj.imp().playing.get() {
                            obj.imp().frame_num.set((obj.imp().frame_num.get() + 1) % totalframe);
                            obj.invalidate_contents();
                        }
                        glib::Continue(true)
                }),
            );

            std::thread::spawn(move || loop {
                std::thread::sleep(std::time::Duration::from_millis(30));
                sender.send(());
            });
        }
    }
    impl MediaFileImpl for LottieAnimation {}
    impl MediaStreamImpl for LottieAnimation {
        fn play(&self, media_stream: &Self::Type) -> bool {
            self.playing.set(true);
            false
        }
    }

    impl gdk::subclass::paintable::PaintableImpl for LottieAnimation {
        fn flags(&self, paintable: &Self::Type) -> gdk::PaintableFlags {
            gdk::PaintableFlags::SIZE
        }

        fn intrinsic_width(&self, _: &Self::Type) -> i32 {
            self.intrinsic.get().0
        }

        fn intrinsic_height(&self, _: &Self::Type) -> i32 {
            self.intrinsic.get().1
        }

        fn intrinsic_aspect_ratio(&self, _: &Self::Type) -> f64 {
            self.intrinsic.get().2
        }

        fn snapshot(
            &self,
            paintable: &Self::Type,
            snapshot: &gdk::Snapshot,
            width: f64,
            height: f64,
        ) {
            self.texture_from_lottie_json()
                .snapshot(snapshot, width, height);
        }
    }

    impl LottieAnimation {
        fn texture_from_lottie_json(&self) -> gdk::Texture {
            if let Some(ref mut animation) = *self.animation.borrow_mut() {
                if let Some(ref mut surface) = *self.surface.borrow_mut() {
                    let frame_num = self.frame_num.get();

                    animation.render(frame_num, surface);

                    let mut data = &mut surface.data();

                    // let bytes = vec![0; data.len() * 4];

                    let mut data = unsafe {
                        std::slice::from_raw_parts_mut(data.as_ptr() as *mut u8, data.len() * 4)
                    };

                    for bgra in data.chunks_exact_mut(4) {
                        (bgra[0], bgra[2]) = (bgra[2], bgra[0]);
                    }

                    let data = glib::Bytes::from_owned(data);

                    let size = animation.size();
                    let (width, height, _) = self.intrinsic.get();

                    let pixbuf = gdk::gdk_pixbuf::Pixbuf::from_bytes(
                        &data,
                        gdk::gdk_pixbuf::Colorspace::Rgb,
                        true,
                        8,
                        width,
                        height,
                        width * 4,
                    );

                    return gdk::Texture::for_pixbuf(&pixbuf);
                }
            }
            panic!();
        }
    }
}

glib::wrapper! {
    pub struct LottieAnimation(ObjectSubclass<imp::LottieAnimation>)
        @extends gtk::MediaFile, gtk::MediaStream,
        // @extends glib::Object,
        @implements gdk::Paintable;
}

impl LottieAnimation {
    pub fn new() -> Self {
        glib::Object::new(&[]).expect("Failed to create LottieAnimation")

        // gtk::MediaFile::for_filename("./data/rotate.webm").
    }
}
