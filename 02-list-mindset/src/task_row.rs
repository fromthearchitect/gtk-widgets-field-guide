use std::cell::Ref;

use gtk::glib::BoxedAnyObject;
use gtk::prelude::*;

use crate::task::Task;

/// Turns one `Task` into one row: a label, set once in `setup` and
/// refreshed on every `bind` as GTK recycles the row for different items.
pub fn factory() -> gtk::SignalListItemFactory {
    let factory = gtk::SignalListItemFactory::new();

    factory.connect_setup(|_, list_item| {
        let label = gtk::Label::builder().xalign(0.0).build();
        list_item
            .downcast_ref::<gtk::ListItem>()
            .unwrap()
            .set_child(Some(&label));
    });

    factory.connect_bind(|_, list_item| {
        let list_item = list_item.downcast_ref::<gtk::ListItem>().unwrap();
        let boxed = list_item.item().and_downcast::<BoxedAnyObject>().unwrap();
        let task: Ref<Task> = boxed.borrow();
        let label = list_item.child().and_downcast::<gtk::Label>().unwrap();

        label.set_label(&if task.done {
            format!("✓ {}", task.title)
        } else {
            task.title.clone()
        });
    });

    factory
}
