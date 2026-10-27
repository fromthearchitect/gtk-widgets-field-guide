use std::cell::Cell;
use std::rc::Rc;

use adw::prelude::*;
use gtk::glib;

use crate::columns;
use crate::photo::{starting_photos, Photo};
use crate::photo_store::PhotoStore;
use crate::tile;

pub fn build(app: &adw::Application) {
    let photos = PhotoStore::new(&starting_photos());

    // The column view comes first, with no model. Clicking a header changes
    // the view's own sorter, a ColumnViewSorter built from the columns'
    // sorters, and nothing re-sorts until that sorter drives a SortListModel.
    // So the view has to exist before the model stack can be built.
    let column_view = gtk::ColumnView::new(None::<gtk::SelectionModel>);
    column_view.set_show_row_separators(true);
    column_view.set_show_column_separators(true);
    column_view.set_enable_rubberband(true);

    let taken = columns::taken();
    column_view.append_column(&columns::name());
    column_view.append_column(&columns::camera());
    column_view.append_column(&taken);
    column_view.append_column(&columns::size());
    column_view.append_column(&columns::rating());

    let sorter = column_view
        .sorter()
        .and_downcast::<gtk::ColumnViewSorter>()
        .unwrap();
    let sort_model = gtk::SortListModel::new(Some(photos.model().clone()), Some(sorter.clone()));
    let selection = gtk::MultiSelection::new(Some(sort_model));
    column_view.set_model(Some(&selection));

    // On a fresh view, the same as a user clicking Taken twice.
    column_view.sort_by_column(Some(&taken), gtk::SortType::Descending);

    // The grid shares the selection model, so it shares the sort and the
    // selection too. Select in one view, switch, and it's still selected.
    let grid_view = gtk::GridView::new(Some(selection.clone()), Some(tile::factory()));
    grid_view.set_min_columns(2);
    grid_view.set_max_columns(6);
    grid_view.set_enable_rubberband(true);

    // As in the filter-sort example, handlers owned by `selection` use the
    // reference the signal passes in rather than capturing it.
    let status = gtk::Label::builder().xalign(0.0).hexpand(true).build();
    let update_status = {
        let (status, sorter) = (status.clone(), sorter.clone());
        Rc::new(move |selection: &gtk::MultiSelection| {
            status.set_label(&format!(
                "{}, {} of {} selected",
                describe_sort(&sorter),
                selection.selection().size(),
                selection.n_items()
            ));
        })
    };
    update_status(&selection);

    let update = update_status.clone();
    selection.connect_selection_changed(move |selection, _, _| update(selection));
    let update = update_status.clone();
    selection.connect_items_changed(move |selection, _, _, _| update(selection));

    // A header click re-sorts without touching the selection model's items,
    // so watch the sorter for that.
    let (update, model) = (update_status.clone(), selection.downgrade());
    sorter.connect_changed(move |sorter, _| {
        // The columns go when the window is destroyed, which empties the
        // sorter and fires one last change. Nothing to report.
        if sorter.n_sort_columns() == 0 {
            return;
        }
        println!(
            "sorter changed: {} (sort columns: {:?})",
            describe_sort(sorter),
            sort_columns(sorter)
        );
        if let Some(model) = model.upgrade() {
            update(&model);
        }
    });

    let rate_up = gtk::Button::with_label("Rate up");
    let model = selection.clone();
    rate_up.connect_clicked(move |_| {
        // Header sorting is still a SortListModel underneath, with no
        // watch-items. The store reports each rating change, so sorted by
        // Rating the rows move without this handler knowing about the sorter.
        for photo in selected_photos(&model) {
            photo.set_rating((photo.rating() + 1).min(5));
        }
    });

    let add = gtk::Button::with_label("Add photo");
    let added = Cell::new(0);
    let store = photos.clone();
    add.connect_clicked(move |_| {
        added.set(added.get() + 1);
        let now = glib::DateTime::now_local().unwrap().to_unix();
        // Appended to the store. Whichever column is sorted, the photo lands
        // in place, in both views.
        store.append(&Photo::new(
            &format!("NEW_{:04}.jpg", added.get()),
            "Pixel 8",
            now,
            3_000_000,
            0,
        ));
    });

    let stack = adw::ViewStack::new();
    stack.add_titled_with_icon(
        &scrolled(&column_view),
        Some("table"),
        "Table",
        "view-list-symbolic",
    );
    stack.add_titled_with_icon(
        &scrolled(&grid_view),
        Some("grid"),
        "Grid",
        "view-grid-symbolic",
    );

    let bar = gtk::Box::builder()
        .spacing(6)
        .margin_top(6)
        .margin_bottom(6)
        .margin_start(12)
        .margin_end(12)
        .build();
    bar.append(&status);
    bar.append(&rate_up);
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
        .title("Columns and Grids")
        .default_width(820)
        .default_height(520)
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

/// The ColumnViewSorter knows which column the user sorted last and in which
/// direction. A header click puts that column first and keeps the earlier
/// ones behind it as tie-breakers; `sort_by_column()` clears them instead.
fn describe_sort(sorter: &gtk::ColumnViewSorter) -> String {
    match sorter.primary_sort_column() {
        Some(column) => {
            let direction = match sorter.primary_sort_order() {
                gtk::SortType::Ascending => "ascending",
                _ => "descending",
            };
            format!(
                "Sorted by {} {direction}",
                column.title().unwrap_or_default()
            )
        }
        None => "Unsorted".to_string(),
    }
}

/// Every column the sorter is using, primary first, each with its direction.
fn sort_columns(sorter: &gtk::ColumnViewSorter) -> Vec<String> {
    (0..sorter.n_sort_columns())
        .filter_map(|i| {
            let (column, order) = sorter.nth_sort_column(i);
            column.map(|column| format!("{} {order:?}", column.title().unwrap_or_default()))
        })
        .collect()
}

/// Positions here are positions in the sorted list, which is what the
/// selection model's bitset refers to.
fn selected_photos(selection: &gtk::MultiSelection) -> Vec<Photo> {
    let bitset = selection.selection();
    let mut photos = Vec::new();

    if let Some((mut iter, first)) = gtk::BitsetIter::init_first(&bitset) {
        let mut position = Some(first);
        while let Some(p) = position {
            photos.push(selection.item(p).and_downcast::<Photo>().unwrap());
            position = iter.next();
        }
    }

    photos
}
