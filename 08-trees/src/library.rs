use gtk::gio;
use gtk::prelude::*;

use crate::node::Node;

enum Entry {
    Folder(&'static str, &'static [Entry]),
    /// Name and size in kB.
    Photo(&'static str, u64),
}

use Entry::{Folder, Photo};

/// Three years of a small photo library. Folders hold folders or photos or
/// both: 2025 has a loose photo next to its trips, and Inbox is an empty
/// folder, which is not the same thing as a photo (it still gets an arrow).
const LIBRARY: &[Entry] = &[
    Folder(
        "2024",
        &[
            Folder(
                "Tasmania",
                &[
                    Photo("DSCF0102.RAF", 24_100),
                    Photo("DSCF0103.jpg", 9_800),
                    Photo("DSCF0141.jpg", 10_600),
                ],
            ),
            Folder(
                "Christmas",
                &[Photo("IMG_1532.HEIC", 2_100), Photo("IMG_1540.HEIC", 1_900)],
            ),
        ],
    ),
    Folder(
        "2025",
        &[
            Folder(
                "Japan",
                &[
                    Photo("DSCF0412.RAF", 24_000),
                    Photo("DSCF0413.jpg", 12_400),
                    Photo("DSCF0588.jpg", 9_100),
                    Photo("IMG_0911.HEIC", 1_800),
                ],
            ),
            Folder(
                "Garden",
                &[Photo("IMG_2041.jpg", 3_200), Photo("IMG_2077.jpg", 2_900)],
            ),
            Photo("IMG_0934.HEIC", 2_100),
        ],
    ),
    Folder(
        "2026",
        &[
            Folder(
                "Blue Mountains",
                &[
                    Photo("DSCF0702.RAF", 23_000),
                    Photo("DSCF0815.jpg", 10_200),
                    Photo("IMG_2290.jpg", 4_600),
                ],
            ),
            Folder("Inbox", &[]),
        ],
    ),
];

/// The root model: a plain `GListStore` of the top-level folders. The tree
/// model wraps this; every level below it is a store owned by its folder.
pub fn starting_library() -> gio::ListStore {
    let root = gio::ListStore::new::<Node>();
    for entry in LIBRARY {
        root.append(&build(entry));
    }
    root
}

fn build(entry: &Entry) -> Node {
    match entry {
        Photo(name, kb) => Node::new_photo(name, kb * 1_000),
        Folder(name, entries) => {
            let folder = Node::new_folder(name);
            for entry in *entries {
                folder.append(&build(entry));
            }
            folder
        }
    }
}

/// Depth-first search by name, for finding the Inbox.
pub fn find(model: &gio::ListStore, name: &str) -> Option<Node> {
    model.iter::<Node>().flatten().find_map(|node| {
        if node.name() == name {
            Some(node)
        } else {
            node.children().and_then(|children| find(&children, name))
        }
    })
}
