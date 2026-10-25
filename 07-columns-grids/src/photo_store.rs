use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use gtk::gio;
use gtk::glib::SignalHandlerId;
use gtk::prelude::*;

use crate::photo::Photo;

/// The store from the items-changed example, for photos. A GListStore that
/// reports when a photo it holds changes, by emitting items_changed(pos, 1, 1)
/// for it, so the SortListModel above re-places that one photo.
///
/// Every column sorts on a different property, so every property is watched.
/// A table makes the watch list longer than a single sorted list does: leave
/// one off and that column goes quietly out of date.
pub struct PhotoStore {
    store: gio::ListStore,
    handlers: RefCell<HashMap<Photo, [SignalHandlerId; 5]>>,
}

impl PhotoStore {
    pub fn new(photos: &[Photo]) -> Rc<Self> {
        let photo_store = Rc::new(Self {
            store: gio::ListStore::new::<Photo>(),
            handlers: RefCell::default(),
        });
        for photo in photos {
            photo_store.append(photo);
        }
        photo_store
    }

    /// What the SortListModel wraps. Add photos through `append`, not on this
    /// directly, or they arrive without handlers.
    pub fn model(&self) -> &gio::ListStore {
        &self.store
    }

    pub fn append(&self, photo: &Photo) {
        let ids = [
            photo.connect_name_notify(self.reporter("name")),
            photo.connect_camera_notify(self.reporter("camera")),
            photo.connect_taken_notify(self.reporter("taken")),
            photo.connect_size_notify(self.reporter("size")),
            photo.connect_rating_notify(self.reporter("rating")),
        ];
        self.handlers.borrow_mut().insert(photo.clone(), ids);
        self.store.append(photo);
    }

    /// The handler lives on the photo, so it holds the store weakly. A strong
    /// reference would be a cycle: store, photo, handler, store.
    fn reporter(&self, property: &'static str) -> impl Fn(&Photo) + 'static {
        let store = self.store.downgrade();
        move |photo| {
            let Some(store) = store.upgrade() else {
                return;
            };
            if let Some(position) = store.find(photo) {
                println!(
                    "{property} changed on {}: items_changed({position}, 1, 1)",
                    photo.name()
                );
                store.items_changed(position, 1, 1);
            }
        }
    }
}

impl Drop for PhotoStore {
    fn drop(&mut self) {
        for (photo, ids) in self.handlers.take() {
            for id in ids {
                photo.disconnect(id);
            }
        }
    }
}
