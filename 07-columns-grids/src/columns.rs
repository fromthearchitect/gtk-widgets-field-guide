use gtk::prelude::*;
use gtk::{glib, pango};

use crate::photo::Photo;

fn property(name: &str) -> gtk::PropertyExpression {
    gtk::PropertyExpression::new(Photo::static_type(), gtk::Expression::NONE, name)
}

/// Each column has its own factory, and each factory builds one cell. The
/// object handed to setup is a ColumnViewCell, not a ListItem, but it has the
/// same `item` property, so the expressions read exactly as they did in a
/// ListView.
fn label_factory(
    xalign: f32,
    label: impl Fn(gtk::Expression) -> gtk::Expression + 'static,
) -> gtk::SignalListItemFactory {
    let factory = gtk::SignalListItemFactory::new();

    factory.connect_setup(move |_, cell| {
        let cell = cell.downcast_ref::<gtk::ColumnViewCell>().unwrap();

        // Ellipsized, or a resizable column can't be dragged narrower than
        // the longest label in it.
        let widget = gtk::Label::builder()
            .xalign(xalign)
            .ellipsize(pango::EllipsizeMode::End)
            .build();
        cell.set_child(Some(&widget));

        label(gtk::ColumnViewCell::this_expression("item").upcast()).bind(
            &widget,
            "label",
            Some(cell),
        );
    });

    factory
}

pub fn name() -> gtk::ColumnViewColumn {
    let factory = label_factory(0.0, |item| item.chain_property::<Photo>("name").upcast());

    // expand shares out any spare width; only Name takes it, so the other
    // columns stay at their natural width as the window grows.
    gtk::ColumnViewColumn::builder()
        .title("Name")
        .factory(&factory)
        .sorter(&gtk::StringSorter::new(Some(property("name"))))
        .expand(true)
        .resizable(true)
        .build()
}

pub fn camera() -> gtk::ColumnViewColumn {
    let factory = label_factory(0.0, |item| item.chain_property::<Photo>("camera").upcast());

    gtk::ColumnViewColumn::builder()
        .title("Camera")
        .factory(&factory)
        .sorter(&gtk::StringSorter::new(Some(property("camera"))))
        .resizable(true)
        .build()
}

/// Displayed as a date, sorted as the number underneath.
pub fn taken() -> gtk::ColumnViewColumn {
    let factory = label_factory(0.0, |item| {
        item.chain_property::<Photo>("taken")
            .chain_closure::<String>(glib::closure!(|_: Option<glib::Object>, taken: i64| {
                glib::DateTime::from_unix_local(taken)
                    .and_then(|date| date.format("%e %b %Y"))
                    .map(|text| text.trim().to_string())
                    .unwrap_or_default()
            }))
            .upcast()
    });

    gtk::ColumnViewColumn::builder()
        .title("Taken")
        .factory(&factory)
        .sorter(&gtk::NumericSorter::new(Some(property("taken"))))
        .resizable(true)
        .build()
}

/// The sorter reads `size`, the bytes, not the formatted label. A
/// StringSorter on the label would put "12.4 MB" above "9.1 MB".
pub fn size() -> gtk::ColumnViewColumn {
    let factory = label_factory(1.0, |item| {
        item.chain_property::<Photo>("size")
            .chain_closure::<String>(glib::closure!(|_: Option<glib::Object>, size: u64| {
                glib::format_size(size).to_string()
            }))
            .upcast()
    });

    gtk::ColumnViewColumn::builder()
        .title("Size")
        .factory(&factory)
        .sorter(&gtk::NumericSorter::new(Some(property("size"))))
        .fixed_width(96)
        .build()
}

pub fn rating() -> gtk::ColumnViewColumn {
    let factory = label_factory(0.0, |item| {
        item.chain_property::<Photo>("rating")
            .chain_closure::<String>(glib::closure!(
                |_: Option<glib::Object>, rating: u32| stars(rating)
            ))
            .upcast()
    });

    gtk::ColumnViewColumn::builder()
        .title("Rating")
        .factory(&factory)
        .sorter(&gtk::NumericSorter::new(Some(property("rating"))))
        .build()
}

pub fn stars(rating: u32) -> String {
    let rating = rating.min(5) as usize;
    format!("{}{}", "★".repeat(rating), "☆".repeat(5 - rating))
}
