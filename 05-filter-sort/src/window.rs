use std::cell::Cell;
use std::rc::Rc;

use adw::prelude::*;
use gtk::gio;

use crate::filters;
use crate::sorters;
use crate::task::{starting_tasks, Task};
use crate::task_row;

pub fn build(app: &adw::Application) {
    let store = gio::ListStore::new::<Task>();
    store.extend_from_slice(&starting_tasks());

    // The stack, bottom to top: store, filter, sort, selection. Each layer is
    // a GListModel wrapping the one below, and the view sees only the top.
    let search = filters::search();
    let every = gtk::EveryFilter::new();
    every.append(search.clone());

    let filter_model = gtk::FilterListModel::new(Some(store.clone()), Some(every.clone()));
    filter_model.set_watch_items(true);
    let sort_model =
        gtk::SortListModel::new(Some(filter_model.clone()), Some(sorters::by_index(0)));
    let selection = gtk::MultiSelection::new(Some(sort_model.clone()));

    // The handlers below are owned by `selection`, so they mustn't capture a
    // strong `selection` themselves: that's a cycle, and the whole stack under
    // it would never be freed. They use the reference the signal passes in
    // instead. The label and the store are safe to capture, since neither of
    // them holds the selection.
    let status = gtk::Label::builder().xalign(0.0).hexpand(true).build();
    let update_status = {
        let (status, store) = (status.clone(), store.clone());
        Rc::new(move |selection: &gtk::MultiSelection| {
            status.set_label(&format!(
                "{} of {} shown, {} selected",
                selection.n_items(),
                store.n_items(),
                selection.selection().size()
            ));
        })
    };
    update_status(&selection);

    let update = update_status.clone();
    selection.connect_selection_changed(move |selection, _, _| {
        println!("selection-changed: {:?}", selected_titles(selection));
        update(selection);
    });
    let update = update_status.clone();
    selection.connect_items_changed(move |selection, position, removed, added| {
        println!(
            "items-changed({position}, -{removed}, +{added}): still selected {:?}",
            selected_titles(selection)
        );
        update(selection);
    });

    let entry = gtk::SearchEntry::builder()
        .placeholder_text("Search titles")
        .build();
    let filter = search.clone();
    entry.connect_search_changed(move |entry| {
        filter.set_search(Some(&entry.text()));
    });

    // Hiding is adding the filter to the EveryFilter, showing is removing it.
    // The EveryFilter tells the FilterListModel either way.
    let hide = gtk::ToggleButton::with_label("Hide done");
    let not_done = filters::not_done();
    hide.connect_toggled(move |button| {
        if button.is_active() {
            every.append(not_done.clone());
        } else {
            every.remove(every.n_items() - 1);
        }
    });

    // GTK defaults watch-items to off; this example turns it on. Switch it
    // off, hide done tasks, then mark one done: the row stays on screen until
    // something else makes the filter run again.
    let watch = gtk::ToggleButton::builder()
        .label("Watch items")
        .active(true)
        .build();
    let model = filter_model.clone();
    watch.connect_toggled(move |button| model.set_watch_items(button.is_active()));

    let sort = gtk::DropDown::from_strings(sorters::NAMES);
    let model = sort_model.clone();
    sort.connect_selected_notify(move |dropdown| {
        model.set_sorter(Some(&sorters::by_index(dropdown.selected())));
    });

    let mark_done = gtk::Button::with_label("Mark done");
    let (model, sorted) = (selection.clone(), sort_model.clone());
    mark_done.connect_clicked(move |_| {
        // Collect first, then mutate. With watch-items on, each set_done can
        // filter a row out and shift every position after it.
        for task in selected_tasks(&model) {
            task.set_done(true);
        }
        // SortListModel has no watch-items, so "Open first" would leave the
        // newly done tasks where they were. Tell the sorter its answers moved.
        if let Some(sorter) = sorted.sorter() {
            sorter.changed(gtk::SorterChange::Different);
        }
    });

    let add = gtk::Button::with_label("Add task");
    let added = Cell::new(0);
    let model = store.clone();
    add.connect_clicked(move |_| {
        added.set(added.get() + 1);
        // Appended to the store, not inserted at a sorted position. The
        // models above put it where it belongs.
        model.append(&Task::new(&format!("New task {}", added.get()), false, 2));
    });

    let list_view = gtk::ListView::new(Some(selection), Some(task_row::factory()));
    let scrolled = gtk::ScrolledWindow::builder()
        .child(&list_view)
        .vexpand(true)
        .build();

    let bar = gtk::Box::builder()
        .spacing(6)
        .margin_top(6)
        .margin_bottom(6)
        .margin_start(12)
        .margin_end(12)
        .build();
    bar.append(&status);
    bar.append(&hide);
    bar.append(&watch);
    bar.append(&sort);
    bar.append(&mark_done);
    bar.append(&add);

    let page = gtk::Box::new(gtk::Orientation::Vertical, 0);
    page.append(&scrolled);
    page.append(&gtk::Separator::new(gtk::Orientation::Horizontal));
    page.append(&bar);

    let header = adw::HeaderBar::new();
    header.set_title_widget(Some(&entry));

    let toolbar_view = adw::ToolbarView::new();
    toolbar_view.add_top_bar(&header);
    toolbar_view.set_content(Some(&page));

    let window = adw::ApplicationWindow::builder()
        .application(app)
        .title("Filtering and Sorting")
        .default_width(820)
        .default_height(480)
        .content(&toolbar_view)
        .build();

    window.present();
}

/// The selected tasks, looked up through the selection model. Its positions
/// are positions in the sorted, filtered list; they mean nothing to the store.
fn selected_tasks(selection: &gtk::MultiSelection) -> Vec<Task> {
    let bitset = selection.selection();
    let mut tasks = Vec::new();

    if let Some((mut iter, first)) = gtk::BitsetIter::init_first(&bitset) {
        let mut position = Some(first);
        while let Some(p) = position {
            tasks.push(selection.item(p).and_downcast::<Task>().unwrap());
            position = iter.next();
        }
    }

    tasks
}

fn selected_titles(selection: &gtk::MultiSelection) -> Vec<String> {
    selected_tasks(selection).iter().map(Task::title).collect()
}
