use std::cell::Ref;

use gtk::glib::BoxedAnyObject;
use gtk::prelude::*;

use crate::task::Task;

/// setup builds the label once per recycled row slot. bind and unbind are a
/// symmetrical pair: the "dim-label" CSS class bind adds for a done task is
/// exactly what unbind removes, so it never lingers once the row slot gets
/// recycled onto a task that isn't done.
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

        label.set_label(&task.title);

        if task.done {
            label.add_css_class("dim-label");
        }
    });

    factory.connect_unbind(|_, list_item| {
        let list_item = list_item.downcast_ref::<gtk::ListItem>().unwrap();
        let label = list_item.child().and_downcast::<gtk::Label>().unwrap();
        label.remove_css_class("dim-label");
    });

    factory
}
