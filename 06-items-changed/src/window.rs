use std::cell::Cell;
use std::rc::Rc;

use adw::prelude::*;

use crate::filters;
use crate::sorters;
use crate::task::{starting_tasks, Task};
use crate::task_row;
use crate::task_store::TaskStore;

pub fn build(app: &adw::Application) {
    let tasks = TaskStore::new(&starting_tasks());
    let store = tasks.model().clone();

    // The same stack as the last example: store, filter, sort, selection. What
    // changed is underneath. The store reports its own item changes, so the
    // filter model's watch-items is left at GTK's default, off, and nothing
    // calls sorter.changed().
    let search = filters::search();
    let every = gtk::EveryFilter::new();
    every.append(search.clone());

    let filter_model = gtk::FilterListModel::new(Some(store.clone()), Some(every.clone()));
    let sort_model =
        gtk::SortListModel::new(Some(filter_model.clone()), Some(sorters::by_index(0)));
    let selection = gtk::MultiSelection::new(Some(sort_model.clone()));

    // Logged at the bottom of the stack as well as the top, so you can compare
    // what the store said with what reached the view.
    store.connect_items_changed(|_, position, removed, added| {
        println!("  store items-changed({position}, -{removed}, +{added})");
    });

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
            "view items-changed({position}, -{removed}, +{added}): still selected {:?}",
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

    let hide = gtk::ToggleButton::with_label("Hide done");
    let not_done = filters::not_done();
    hide.connect_toggled(move |button| {
        if button.is_active() {
            every.append(not_done.clone());
        } else {
            every.remove(every.n_items() - 1);
        }
    });

    // Switch it off, sort by Open first, then mark a task done: it stays where
    // it was, as it did before the last example's sorter.changed() call.
    let report = gtk::ToggleButton::builder()
        .label("Report changes")
        .active(true)
        .build();
    let task_store = tasks.clone();
    report.connect_toggled(move |button| task_store.set_reporting(button.is_active()));

    let sort = gtk::DropDown::from_strings(sorters::NAMES);
    let model = sort_model.clone();
    sort.connect_selected_notify(move |dropdown| {
        model.set_sorter(Some(&sorters::by_index(dropdown.selected())));
    });

    // Collect first, then mutate: each set_done can filter a row out and shift
    // every position after it. No sorter.changed() afterwards any more.
    let mark_done = gtk::Button::with_label("Mark done");
    let model = selection.clone();
    mark_done.connect_clicked(move |_| {
        for task in selected_tasks(&model) {
            task.set_done(true);
        }
    });

    let raise = gtk::Button::with_label("Raise to P1");
    let model = selection.clone();
    raise.connect_clicked(move |_| {
        for task in selected_tasks(&model) {
            task.set_priority(1);
        }
    });

    // A stand-in for a sync or an undo: something that changes a task without
    // going through the selection or any button handler that knows about the
    // sorter. It completes the first open task in store order.
    let sync = gtk::Button::builder()
        .label("Simulate sync")
        .tooltip_text("Completes the first open task, as a sync from another device would")
        .build();
    let model = store.clone();
    sync.connect_clicked(move |_| {
        let open = model
            .iter::<Task>()
            .map(Result::unwrap)
            .find(|task| !task.done());
        if let Some(task) = open {
            println!("sync completes {:?}", task.title());
            task.set_done(true);
        }
    });

    let add = gtk::Button::with_label("Add task");
    let added = Cell::new(0);
    let task_store = tasks.clone();
    add.connect_clicked(move |_| {
        added.set(added.get() + 1);
        task_store.append(&Task::new(&format!("New task {}", added.get()), false, 2));
    });

    let delete = gtk::Button::with_label("Delete");
    let (model, task_store) = (selection.clone(), tasks.clone());
    delete.connect_clicked(move |_| {
        for task in selected_tasks(&model) {
            task_store.remove(&task);
        }
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
    bar.append(&report);
    bar.append(&sort);
    bar.append(&mark_done);
    bar.append(&raise);
    bar.append(&sync);
    bar.append(&add);
    bar.append(&delete);

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
        .title("Lists That Notice")
        .default_width(1040)
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
