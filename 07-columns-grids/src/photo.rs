use gtk::glib;

mod imp {
    use std::cell::{Cell, RefCell};

    use gtk::glib;
    use gtk::glib::Properties;
    use gtk::prelude::*;
    use gtk::subclass::prelude::*;

    #[derive(Default, Properties)]
    #[properties(wrapper_type = super::Photo)]
    pub struct Photo {
        #[property(get, set)]
        name: RefCell<String>,
        #[property(get, set)]
        camera: RefCell<String>,
        /// Unix seconds. Stored raw and formatted only for display, so the
        /// column can sort on the number.
        #[property(get, set)]
        taken: Cell<i64>,
        /// Bytes, for the same reason.
        #[property(get, set)]
        size: Cell<u64>,
        #[property(get, set, maximum = 5)]
        rating: Cell<u32>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Photo {
        const NAME: &'static str = "FieldGuidePhoto";
        type Type = super::Photo;
    }

    #[glib::derived_properties]
    impl ObjectImpl for Photo {}
}

glib::wrapper! {
    pub struct Photo(ObjectSubclass<imp::Photo>);
}

impl Photo {
    pub fn new(name: &str, camera: &str, taken: i64, size: u64, rating: u32) -> Self {
        glib::Object::builder()
            .property("name", name)
            .property("camera", camera)
            .property("taken", taken)
            .property("size", size)
            .property("rating", rating)
            .build()
    }
}

/// Twelve photos from three cameras: name, camera, taken, size in kB, rating.
/// The sizes straddle 10 MB on purpose: sorted as display strings, "12.4 MB"
/// comes before "9.1 MB".
const PHOTOS: &[(&str, &str, i64, u64, u32)] = &[
    ("IMG_2041.jpg", "Pixel 8", 1_735_689_600, 3_200, 3),
    ("IMG_2077.jpg", "Pixel 8", 1_738_368_000, 2_900, 1),
    ("DSCF0412.RAF", "Fujifilm X-T5", 1_740_787_200, 24_000, 5),
    ("DSCF0413.jpg", "Fujifilm X-T5", 1_740_787_260, 12_400, 4),
    ("IMG_0911.HEIC", "iPhone 15", 1_743_465_600, 1_800, 2),
    ("IMG_0934.HEIC", "iPhone 15", 1_746_057_600, 2_100, 0),
    ("DSCF0588.jpg", "Fujifilm X-T5", 1_748_736_000, 9_100, 3),
    ("IMG_2290.jpg", "Pixel 8", 1_751_328_000, 4_600, 4),
    ("DSCF0702.RAF", "Fujifilm X-T5", 1_754_006_400, 23_000, 2),
    ("IMG_1022.HEIC", "iPhone 15", 1_756_684_800, 2_400, 5),
    ("IMG_2355.jpg", "Pixel 8", 1_759_276_800, 3_700, 1),
    ("DSCF0815.jpg", "Fujifilm X-T5", 1_761_955_200, 10_200, 3),
];

pub fn starting_photos() -> Vec<Photo> {
    PHOTOS
        .iter()
        .map(|&(name, camera, taken, kb, rating)| {
            Photo::new(name, camera, taken, kb * 1_000, rating)
        })
        .collect()
}
