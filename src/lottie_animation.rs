use glib::clone;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gdk, gio, glib};

use rlottie;

use flate2::read::GzDecoder;
use std::io;
use std::io::prelude::*;

mod imp {
    use super::*;
    use std::cell::{Cell, RefCell};

    #[derive(Default)]
    pub struct LottieAnimation {
        pub frame_num: Cell<usize>,
        pub playing: Cell<bool>,
        pub animation: RefCell<Option<rlottie::Animation>>,
        pub surface: RefCell<Option<rlottie::Surface>>,
        pub intrinsic: Cell<(i32, i32, f64)>,
        pub texture: RefCell<Option<gdk::MemoryTexture>>,

        pub player_source_id: Cell<Option<glib::SourceId>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for LottieAnimation {
        const NAME: &'static str = "ContentLottieAnimation";
        type Type = super::LottieAnimation;
        type ParentType = gtk::MediaFile;
        type Interfaces = (gdk::Paintable,);
    }

    impl ObjectImpl for LottieAnimation {}
    impl MediaFileImpl for LottieAnimation {
        fn open(&self, media_file: &Self::Type) {
            if let Some(file) = media_file.file() {
                let path = file.path().unwrap();
                let animation = match path
                    .extension()
                    .unwrap_or_default()
                    .to_str()
                    .unwrap_or_default()
                {
                    "json" => rlottie::Animation::from_file(path).expect("Can't open animation"),
                    "tgs" => {
                        let data = file.load_contents(gio::Cancellable::NONE).unwrap().0;

                        let mut gz = GzDecoder::new(&*data);

                        let mut buf = String::new();

                        gz.read_to_string(&mut buf).expect("can't read file");

                        rlottie::Animation::from_data(
                            buf,
                            path.file_name().unwrap().to_str().unwrap(), // path.file_name().unwrap().to_str().unwrap(),
                            "",
                        )
                        .expect("Can't create tgs animation")
                    }
                    _ => panic!("unsupporded file type"),
                };

                let size = animation.size();
                let framerate = animation.framerate();
                _ = self.animation.replace(Some(animation));

                let (width, height) = (size.width as i32, size.height as i32);
                let aspect_ratio = width as f64 / height as f64;

                self.intrinsic.set((width, height, aspect_ratio));

                let surface = rlottie::Surface::new(size);

                self.surface.replace(Some(surface));
            }
        }
    }
    impl MediaStreamImpl for LottieAnimation {
        fn play(&self, obj: &Self::Type) -> bool {
            let id = self.player_source_id.take();
            if id.is_some() {
                self.player_source_id.set(id);
                false
            } else {
                let id = glib::timeout_add_local(
                    std::time::Duration::from_secs_f64(1.0 / 60.0),
                    clone!(@weak obj => @default-return glib::Continue(false), move || {
                        obj.imp().setup_next_frame();
                        obj.invalidate_contents();
                        glib::Continue(true)
                    }),
                );
                self.player_source_id.set(Some(id));
                true
            }
        }

        fn pause(&self, media_stream: &Self::Type) {
            if let Some(id) = self.player_source_id.take() {
                id.remove();
            }
        }
    }

    impl gdk::subclass::paintable::PaintableImpl for LottieAnimation {
        fn flags(&self, _: &Self::Type) -> gdk::PaintableFlags {
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

        fn snapshot(&self, _: &Self::Type, snapshot: &gdk::Snapshot, width: f64, height: f64) {
            if let Some(texture) = &*self.texture.borrow() {
                texture.snapshot(snapshot, width, height);
            }
        }
    }

    impl LottieAnimation {
        fn texture_from_bytes(data: &[u8], width: i32, height: i32) -> Option<gdk::MemoryTexture> {
            let data = unsafe {
                glib::translate::from_glib_full(glib::ffi::g_bytes_new_with_free_func(
                    data.as_ptr() as *const _,
                    data.len(),
                    glib::ffi::GDestroyNotify::None,
                    0 as *mut _,
                ))
            };

            let texture = gdk::MemoryTexture::new(
                width,
                height,
                gdk::MemoryFormat::B8g8r8a8,
                &data,
                width as usize * 4,
            );

            Some(texture)
        }

        fn setup_next_frame(&self) {
            if let Some(ref mut animation) = *self.animation.borrow_mut() {
                if let Some(ref mut surface) = *self.surface.borrow_mut() {
                    let frame_num = self.frame_num.get();

                    animation.render(frame_num, surface);

                    self.frame_num.set((frame_num + 1) % animation.totalframe());

                    let (width, height, _) = self.intrinsic.get();

                    let data = surface.data();

                    let mut data = unsafe {
                        std::slice::from_raw_parts_mut(data.as_ptr() as *mut u8, data.len() * 4)
                    };

                    let texture = Self::texture_from_bytes(&data, width, height);

                    self.texture.replace(texture);
                }
            }
        }
    }
}

glib::wrapper! {
    pub struct LottieAnimation(ObjectSubclass<imp::LottieAnimation>)
        @extends gtk::MediaFile, gtk::MediaStream,
        @implements gdk::Paintable;
}

impl LottieAnimation {
    pub fn from_file(file: gio::File) -> Self {
        glib::Object::new(&[("file", &file)]).expect("Failed to create LottieAnimation")
    }

    pub fn from_filename(path: &str) -> Self {
        let file = gio::File::for_path("./data/animations/AuthorizationStateWaitCode.tgs");
        Self::from_file(file)
    }
}
