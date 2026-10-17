use gtk::glib;
use gtk::prelude::*;

use crate::task::Task;

/// Bound with expressions, as in the last example, so marking a task done
/// updates its row without the factory. The bind and unbind handlers are here
/// only to log: an items_changed(pos, 1, 1) tells the view the item at `pos`
/// was replaced, and the log shows what the view does about that.
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

    factory.connect_bind(|_, list_item| println!("    bind {:?}", title_of(list_item)));
    factory.connect_unbind(|_, list_item| println!("    unbind {:?}", title_of(list_item)));

    factory
}

fn title_of(list_item: &glib::Object) -> Option<String> {
    list_item
        .downcast_ref::<gtk::ListItem>()
        .and_then(|list_item| list_item.item())
        .and_downcast::<Task>()
        .map(|task| task.title())
}
