use gtk::prelude::*;
use gtk::{glib, pango};

use crate::node::Node;

/// Sorters compare Nodes, not rows: `TreeListRowSorter` unwraps the rows
/// before handing them on. So these read exactly as they did in the
/// columns-grids example.
fn property(name: &str) -> gtk::PropertyExpression {
    gtk::PropertyExpression::new(Node::static_type(), gtk::Expression::NONE, name)
}

/// The cell's `item` is a `TreeListRow`, not a Node, because the tree model
/// was built with passthrough off. The Node is one more hop, through the
/// row's own `item` property.
fn node() -> gtk::PropertyExpression {
    gtk::ColumnViewCell::this_expression("item").chain_property::<gtk::TreeListRow>("item")
}

/// Name is the tree column. The cell holds a `TreeExpander`, which draws the
/// indent and the arrow, and the expander wraps the actual content. Only one
/// column gets an expander; the rest are ordinary cells.
pub fn name() -> gtk::ColumnViewColumn {
    let factory = gtk::SignalListItemFactory::new();

    factory.connect_setup(|_, cell| {
        let cell = cell.downcast_ref::<gtk::ColumnViewCell>().unwrap();

        let icon = gtk::Image::new();
        let label = gtk::Label::builder()
            .xalign(0.0)
            .ellipsize(pango::EllipsizeMode::End)
            .build();
        let content = gtk::Box::new(gtk::Orientation::Horizontal, 6);
        content.append(&icon);
        content.append(&label);

        let expander = gtk::TreeExpander::builder().child(&content).build();
        cell.set_child(Some(&expander));

        // The expander needs the row, not the node: depth and expanded
        // state live on the row. Binding `item` straight to `list-row` is
        // what GTK's own demos do, and it clears on unbind for free.
        gtk::ColumnViewCell::this_expression("item").bind(&expander, "list-row", Some(cell));

        let node = node();
        node.chain_property::<Node>("name")
            .bind(&label, "label", Some(cell));
        node.chain_property::<Node>("is-folder")
            .chain_closure::<String>(glib::closure!(|_: Option<glib::Object>, folder: bool| {
                if folder {
                    "folder-symbolic"
                } else {
                    "image-x-generic-symbolic"
                }
                .to_string()
            }))
            .bind(&icon, "icon-name", Some(cell));
    });

    gtk::ColumnViewColumn::builder()
        .title("Name")
        .factory(&factory)
        .sorter(&gtk::StringSorter::new(Some(property("name"))))
        .expand(true)
        .resizable(true)
        .build()
}

/// A plain cell: no expander, so the label sits at the column's left edge
/// whatever the row's depth. The indent belongs to the Name column alone.
fn label_factory(
    xalign: f32,
    label: impl Fn(gtk::PropertyExpression) -> gtk::Expression + 'static,
) -> gtk::SignalListItemFactory {
    let factory = gtk::SignalListItemFactory::new();

    factory.connect_setup(move |_, cell| {
        let cell = cell.downcast_ref::<gtk::ColumnViewCell>().unwrap();

        let widget = gtk::Label::builder()
            .xalign(xalign)
            .ellipsize(pango::EllipsizeMode::End)
            .build();
        cell.set_child(Some(&widget));

        label(node()).bind(&widget, "label", Some(cell));
    });

    factory
}

/// "Folder", or the file extension. Two inputs, so a ClosureExpression
/// rather than a chain. The sorter uses the same text: sorted by Kind, the
/// folders group together and the photos group by format.
fn kind_of(is_folder: gtk::Expression, name: gtk::Expression) -> gtk::Expression {
    gtk::ClosureExpression::new::<String>(
        [is_folder, name],
        glib::closure!(|_: Option<glib::Object>, folder: bool, name: String| {
            if folder {
                "Folder".to_string()
            } else {
                name.rsplit('.').next().unwrap_or_default().to_uppercase()
            }
        }),
    )
    .upcast()
}

pub fn kind() -> gtk::ColumnViewColumn {
    let factory = label_factory(0.0, |node| {
        kind_of(
            node.chain_property::<Node>("is-folder").upcast(),
            node.chain_property::<Node>("name").upcast(),
        )
    });
    let sorter = gtk::StringSorter::new(Some(kind_of(
        property("is-folder").upcast(),
        property("name").upcast(),
    )));

    gtk::ColumnViewColumn::builder()
        .title("Kind")
        .factory(&factory)
        .sorter(&sorter)
        .resizable(true)
        .build()
}

/// A folder's size is the total underneath it, so sorting a level by size
/// puts the biggest trip first and the biggest photo first inside it.
pub fn size() -> gtk::ColumnViewColumn {
    let factory = label_factory(1.0, |node| {
        node.chain_property::<Node>("size")
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
