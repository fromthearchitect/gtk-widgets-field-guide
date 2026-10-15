use gtk::glib;
use gtk::prelude::*;

use crate::task::Task;

/// No bind or unbind handlers this time. setup wires each widget to an
/// expression rooted at the ListItem's `item` property, and GTK re-evaluates
/// the expressions as the slot is recycled onto a new item or the item's
/// properties change. Marking a task done updates its row without the factory
/// being involved.
pub fn factory() -> gtk::SignalListItemFactory {
    let factory = gtk::SignalListItemFactory::new();

    factory.connect_setup(|_, list_item| {
        let list_item = list_item.downcast_ref::<gtk::ListItem>().unwrap();

        let title = gtk::Label::builder().xalign(0.0).hexpand(true).build();
        let done = gtk::Image::from_icon_name("object-select-symbolic");
        let priority = gtk::Label::new(None);
        priority.add_css_class("dim-label");
        priority.add_css_class("numeric");

        let row = gtk::Box::builder().spacing(12).build();
        row.append(&title);
        row.append(&done);
        row.append(&priority);
        list_item.set_child(Some(&row));

        let item = gtk::ListItem::this_expression("item");
        item.chain_property::<Task>("title")
            .bind(&title, "label", Some(list_item));
        item.chain_property::<Task>("done")
            .bind(&done, "visible", Some(list_item));
        item.chain_property::<Task>("priority")
            .chain_closure::<String>(glib::closure!(
                |_: Option<glib::Object>, priority: u32| format!("P{priority}")
            ))
            .bind(&priority, "label", Some(list_item));
    });

    factory
}
