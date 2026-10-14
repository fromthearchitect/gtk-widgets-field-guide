use std::cell::Ref;

use adw::prelude::*;
use gtk::gio;
use gtk::glib::BoxedAnyObject;

use crate::single::page_layout;
use crate::task::Task;
use crate::task_row;

/// Batch selection. MultiSelection has no selected-items property, so the
/// status label re-queries the model's bitset on every selection-changed.
pub fn page(store: &gio::ListStore) -> gtk::Widget {
    let selection = gtk::MultiSelection::new(Some(store.clone()));

    let status = gtk::Label::builder()
        .label("0 selected")
        .xalign(0.0)
        .wrap(true)
        .hexpand(true)
        .build();

    let label = status.clone();
    selection.connect_selection_changed(move |selection, position, n_items| {
        // position and n_items describe the range that CHANGED,
        // not the range that is now selected.
        let titles = selected_titles(selection);
        println!(
            "args = ({position}, {n_items}),  selection size = {}",
            titles.len()
        );
        label.set_label(&format!("{} selected", titles.len()));
    });

    let all = gtk::Button::with_label("Select all");
    let model = selection.clone();
    all.connect_clicked(move |_| {
        model.select_all();
    });

    let range = gtk::Button::with_label("Select first five");
    let model = selection.clone();
    range.connect_clicked(move |_| {
        let changed = model.select_range(0, 5, false);
        println!("MultiSelection select_range(0, 5, false) returned {changed}");
    });

    let clear = gtk::Button::with_label("Clear");
    let model = selection.clone();
    clear.connect_clicked(move |_| {
        model.unselect_all();
    });

    let list_view = gtk::ListView::new(Some(selection), Some(task_row::factory()));
    page_layout(&list_view, &status, &[&all, &range, &clear])
}

/// What's actually selected, read from the model rather than from the signal's
/// arguments. Positions are looked up through the selection model, not the
/// store: they only agree until a filter or sort model joins the stack.
fn selected_titles(selection: &gtk::MultiSelection) -> Vec<String> {
    let bitset = selection.selection();
    let mut titles = Vec::new();

    if let Some((mut iter, first)) = gtk::BitsetIter::init_first(&bitset) {
        let mut position = Some(first);
        while let Some(p) = position {
            let item = selection.item(p).unwrap();
            let boxed = item.downcast::<BoxedAnyObject>().unwrap();
            let task: Ref<Task> = boxed.borrow();
            titles.push(task.title.clone());
            position = iter.next();
        }
    }

    titles
}
