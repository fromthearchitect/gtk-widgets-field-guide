use gtk::prelude::*;
use gtk::{glib, pango};

use crate::columns::stars;
use crate::photo::Photo;

/// The grid's factory. A GridView hands setup a plain ListItem, one per tile,
/// and lays the tiles out in rows itself. There are no columns to configure
/// and no headers to click, so the grid sorts however the model under it is
/// sorted.
pub fn factory() -> gtk::SignalListItemFactory {
    let factory = gtk::SignalListItemFactory::new();

    factory.connect_setup(|_, list_item| {
        let list_item = list_item.downcast_ref::<gtk::ListItem>().unwrap();

        let image = gtk::Image::builder()
            .icon_name("image-x-generic-symbolic")
            .pixel_size(64)
            .build();
        let name = gtk::Label::builder()
            .ellipsize(pango::EllipsizeMode::Middle)
            .max_width_chars(14)
            .build();
        let rating = gtk::Label::new(None);
        rating.add_css_class("dim-label");

        let tile = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(6)
            .margin_top(12)
            .margin_bottom(12)
            .margin_start(12)
            .margin_end(12)
            .build();
        tile.append(&image);
        tile.append(&name);
        tile.append(&rating);
        list_item.set_child(Some(&tile));

        let item = gtk::ListItem::this_expression("item");
        item.chain_property::<Photo>("name")
            .bind(&name, "label", Some(list_item));
        item.chain_property::<Photo>("rating")
            .chain_closure::<String>(glib::closure!(
                |_: Option<glib::Object>, rating: u32| stars(rating)
            ))
            .bind(&rating, "label", Some(list_item));
    });

    factory
}
