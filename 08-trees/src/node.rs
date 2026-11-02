use gtk::gio;
use gtk::glib;
use gtk::subclass::prelude::*;

mod imp {
    use std::cell::{Cell, RefCell};

    use gtk::gio;
    use gtk::glib;
    use gtk::glib::{Properties, WeakRef};
    use gtk::prelude::*;
    use gtk::subclass::prelude::*;

    #[derive(Default, Properties)]
    #[properties(wrapper_type = super::Node)]
    pub struct Node {
        #[property(get, set)]
        name: RefCell<String>,
        /// Bytes. A photo's own size; for a folder, the total of everything
        /// under it.
        #[property(get, set)]
        size: Cell<u64>,
        #[property(get, name = "is-folder")]
        pub(super) is_folder: Cell<bool>,
        /// A folder's children. The node owns this store; `TreeListModel`
        /// only borrows it while the row is expanded. Collapsing a row drops
        /// the model it was given, and expanding asks for it again, so a
        /// store created fresh in that callback would forget any photo added
        /// while the folder was closed.
        pub(super) children: RefCell<Option<gio::ListStore>>,
        /// Weak, or every child would keep its folder alive and the folder
        /// keeps its children alive: a cycle.
        pub(super) parent: WeakRef<super::Node>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Node {
        const NAME: &'static str = "FieldGuideNode";
        type Type = super::Node;
    }

    #[glib::derived_properties]
    impl ObjectImpl for Node {}
}

glib::wrapper! {
    pub struct Node(ObjectSubclass<imp::Node>);
}

impl Node {
    pub fn new_photo(name: &str, size: u64) -> Self {
        glib::Object::builder()
            .property("name", name)
            .property("size", size)
            .build()
    }

    pub fn new_folder(name: &str) -> Self {
        let node: Self = glib::Object::builder().property("name", name).build();
        node.imp().is_folder.set(true);
        node.imp()
            .children
            .replace(Some(gio::ListStore::new::<Node>()));
        node
    }

    /// What `TreeListModel` shows under this node: the folder's own store,
    /// the same object every time it asks, or `None` for a photo.
    pub fn children(&self) -> Option<gio::ListStore> {
        self.imp().children.borrow().clone()
    }

    pub fn parent(&self) -> Option<Node> {
        self.imp().parent.upgrade()
    }

    /// Appends `child` to this folder and adds its size to this folder and
    /// every folder above it. Plain property changes: the labels bound to
    /// `size` update, and nothing is re-sorted (see the README).
    pub fn append(&self, child: &Node) {
        let children = self.children().expect("append on a photo");
        child.imp().parent.set(Some(self));
        children.append(child);

        let added = child.size();
        let mut folder = Some(self.clone());
        while let Some(f) = folder {
            f.set_size(f.size() + added);
            folder = f.parent();
        }
    }
}
