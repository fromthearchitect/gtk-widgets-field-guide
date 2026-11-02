use gtk::prelude::*;
use gtk::{glib, pango};

use crate::node::Node;

/// The same tree in a `ListView`: one `TreeExpander` per row, with the row's
/// content as its child. No columns, no headers, so nothing to click to sort
/// by. It sorts however the model under it is sorted, which is the table's
/// sort, because the two views share one model stack.
pub fn factory() -> gtk::SignalListItemFactory {
    let factory = gtk::SignalListItemFactory::new();

    factory.connect_setup(|_, list_item| {
        let list_item = list_item.downcast_ref::<gtk::ListItem>().unwrap();

        let name = gtk::Label::builder()
            .xalign(0.0)
            .hexpand(true)
            .ellipsize(pango::EllipsizeMode::End)
            .build();
        let size = gtk::Label::new(None);
        size.add_css_class("dim-label");
        size.add_css_class("numeric");

        let content = gtk::Box::new(gtk::Orientation::Horizontal, 12);
        content.append(&name);
        content.append(&size);

        let expander = gtk::TreeExpander::builder().child(&content).build();
        list_item.set_child(Some(&expander));

        // A ListView hands setup a ListItem; its `item` is the TreeListRow.
        let item = gtk::ListItem::this_expression("item");
        item.bind(&expander, "list-row", Some(list_item));

        let node = item.chain_property::<gtk::TreeListRow>("item");
        node.chain_property::<Node>("name")
            .bind(&name, "label", Some(list_item));
        node.chain_property::<Node>("size")
            .chain_closure::<String>(glib::closure!(|_: Option<glib::Object>, size: u64| {
                glib::format_size(size).to_string()
            }))
            .bind(&size, "label", Some(list_item));
    });

    factory
}
