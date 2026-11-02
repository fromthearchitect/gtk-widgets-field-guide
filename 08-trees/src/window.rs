use std::cell::Cell;
use std::rc::Rc;

use adw::prelude::*;

use crate::columns;
use crate::library::{find, starting_library};
use crate::node::Node;
use crate::outline;

pub fn build(app: &adw::Application) {
    let root = starting_library();
    let inbox = find(&root, "Inbox").unwrap();

    // The tree model wraps the root store. The closure is how it finds out
    // what's under an item: a model for a folder, None for a photo. With
    // passthrough off, the items this model hands out are TreeListRows, each
    // wrapping a Node, and that's what every factory and sorter above gets.
    //
    // Run from a terminal to see when the closure is called. It isn't only on
    // expand: the expander asks whether a row *can* expand by calling it too,
    // and drops the answer. Cheap and idempotent, or it hurts.
    let tree = gtk::TreeListModel::new(root.clone(), false, false, |item| {
        let node = item.downcast_ref::<Node>().unwrap();
        println!("children of {}?", node.name());
        node.children().map(|children| children.upcast())
    });

    let column_view = gtk::ColumnView::new(None::<gtk::SelectionModel>);
    column_view.set_enable_rubberband(true);

    let name = columns::name();
    column_view.append_column(&name);
    column_view.append_column(&columns::kind());
    column_view.append_column(&columns::size());

    // The view's sorter compares Nodes, so it can't sort the tree model's
    // rows on its own. TreeListRowSorter wraps it: given two rows, it walks
    // up to the level they share, compares the Nodes there with the inner
    // sorter, and keeps every child under its parent.
    let sorter = column_view
        .sorter()
        .and_downcast::<gtk::ColumnViewSorter>()
        .unwrap();
    let row_sorter = gtk::TreeListRowSorter::new(Some(sorter.clone()));
    let sort_model = gtk::SortListModel::new(Some(tree.clone()), Some(row_sorter));
    let selection = gtk::MultiSelection::new(Some(sort_model));
    column_view.set_model(Some(&selection));
    column_view.sort_by_column(Some(&name), gtk::SortType::Ascending);

    // One selection model, two views: the outline shows the same rows,
    // expanded the same way, sorted the same way, with the same selection.
    let list_view = gtk::ListView::new(Some(selection.clone()), Some(outline::factory()));
    list_view.set_enable_rubberband(true);

    let status = gtk::Label::builder().xalign(0.0).hexpand(true).build();
    let update_status = {
        let (status, sorter) = (status.clone(), sorter.clone());
        Rc::new(move |selection: &gtk::MultiSelection| {
            status.set_label(&format!(
                "{} rows, {} selected, {}",
                selection.n_items(),
                selection.selection().size(),
                describe_sort(&sorter)
            ));
        })
    };
    update_status(&selection);

    let update = update_status.clone();
    selection.connect_selection_changed(move |selection, _, _| update(selection));
    let update = update_status.clone();
    selection.connect_items_changed(move |selection, _, _, _| update(selection));
    let (update, model) = (update_status.clone(), selection.downgrade());
    sorter.connect_changed(move |sorter, _| {
        if sorter.n_sort_columns() == 0 {
            return;
        }
        if let Some(model) = model.upgrade() {
            update(&model);
        }
    });

    // Expanding row i inserts its children at i + 1, and the loop walks
    // straight into them, so one pass opens every level. Positions here are
    // tree positions, not sorted ones: the tree model is asked directly.
    let expand_all = gtk::Button::with_label("Expand all");
    let model = tree.clone();
    expand_all.connect_clicked(move |_| {
        let mut position = 0;
        while let Some(row) = model.row(position) {
            row.set_expanded(true);
            position += 1;
        }
    });

    // Backwards, so collapsing a row only ever removes rows the loop has
    // already passed.
    let collapse_all = gtk::Button::with_label("Collapse all");
    let model = tree.clone();
    collapse_all.connect_clicked(move |_| {
        for position in (0..model.n_items()).rev() {
            if let Some(row) = model.row(position) {
                row.set_expanded(false);
            }
        }
    });

    // Into the selected folder, or the folder of the selected photo, or the
    // Inbox. The photo goes into the folder's own store. If that folder is
    // expanded the new row appears, sorted into place. If it's collapsed
    // nothing shows until it's opened, and then the photo is there, because
    // the folder kept the store the tree model let go of.
    let add = gtk::Button::with_label("Add photo");
    let added = Cell::new(0);
    let (model, sorter) = (selection.clone(), sorter.clone());
    add.connect_clicked(move |_| {
        added.set(added.get() + 1);
        let folder = selected_folder(&model).unwrap_or_else(|| inbox.clone());
        let photo = Node::new_photo(&format!("NEW_{:04}.jpg", added.get()), 3_000_000);
        println!("adding {} to {}", photo.name(), folder.name());
        folder.append(&photo);

        // The folder totals just grew under a sorter that may be reading
        // them, and the sort model's order is now stale. Left alone, sorted
        // by Size, the folder stays put; worse, opening a collapsed folder
        // inserts its rows against the live totals and can put the new photo
        // above its own parent. items_changed(pos, 1, 1) on the folder would
        // re-place it, but TreeListModel treats that as the row removed and
        // a new one inserted, collapsed. So tell the sorter instead: every
        // row is re-sorted in place and nothing closes.
        sorter.changed(gtk::SorterChange::Different);
    });

    let stack = adw::ViewStack::new();
    stack.add_titled_with_icon(
        &scrolled(&column_view),
        Some("table"),
        "Table",
        "view-list-symbolic",
    );
    stack.add_titled_with_icon(
        &scrolled(&list_view),
        Some("outline"),
        "Outline",
        "view-list-bullet-symbolic",
    );

    let bar = gtk::Box::builder()
        .spacing(6)
        .margin_top(6)
        .margin_bottom(6)
        .margin_start(12)
        .margin_end(12)
        .build();
    bar.append(&status);
    bar.append(&expand_all);
    bar.append(&collapse_all);
    bar.append(&add);

    let page = gtk::Box::new(gtk::Orientation::Vertical, 0);
    page.append(&stack);
    page.append(&gtk::Separator::new(gtk::Orientation::Horizontal));
    page.append(&bar);

    let header = adw::HeaderBar::new();
    header.set_title_widget(Some(
        &adw::ViewSwitcher::builder()
            .stack(&stack)
            .policy(adw::ViewSwitcherPolicy::Wide)
            .build(),
    ));

    let toolbar_view = adw::ToolbarView::new();
    toolbar_view.add_top_bar(&header);
    toolbar_view.set_content(Some(&page));

    let window = adw::ApplicationWindow::builder()
        .application(app)
        .title("Trees")
        .default_width(720)
        .default_height(560)
        .content(&toolbar_view)
        .build();

    window.present();
}

fn scrolled(child: &impl IsA<gtk::Widget>) -> gtk::ScrolledWindow {
    gtk::ScrolledWindow::builder()
        .child(child)
        .vexpand(true)
        .build()
}

fn describe_sort(sorter: &gtk::ColumnViewSorter) -> String {
    match sorter.primary_sort_column() {
        Some(column) => {
            let direction = match sorter.primary_sort_order() {
                gtk::SortType::Ascending => "ascending",
                _ => "descending",
            };
            format!(
                "sorted by {} {direction}",
                column.title().unwrap_or_default()
            )
        }
        None => "unsorted".to_string(),
    }
}

/// The first selected row's folder. Selection positions are sorted positions
/// and the items at them are TreeListRows, so the Node is two hops away, and
/// the row's `parent()` is how a photo finds the folder it's in.
fn selected_folder(selection: &gtk::MultiSelection) -> Option<Node> {
    let position = selection.selection().minimum();
    let row = selection
        .item(position)
        .and_downcast::<gtk::TreeListRow>()?;
    let node = row.item().and_downcast::<Node>()?;
    if node.is_folder() {
        Some(node)
    } else {
        row.parent()?.item().and_downcast::<Node>()
    }
}
