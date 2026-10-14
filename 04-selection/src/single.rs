use std::cell::Ref;

use adw::prelude::*;
use gtk::gio;
use gtk::glib::BoxedAnyObject;

use crate::task::Task;
use crate::task_row;

/// Master-detail: the detail label subscribes to the selection model and never
/// learns which widget is displaying it.
pub fn page(store: &gio::ListStore) -> gtk::Widget {
    // Actually starts empty. Installing the model is what triggers the
    // autoselect, so autoselect(false) has to come before model(); the other
    // order selects item 0 while autoselect is still at its default TRUE.
    let selection = gtk::SingleSelection::builder()
        .autoselect(false)
        .can_unselect(true)
        .model(store)
        .build();

    let detail = gtk::Label::builder()
        .label("Nothing selected")
        .xalign(0.0)
        .wrap(true)
        .hexpand(true)
        .build();

    let label = detail.clone();
    selection.connect_selected_item_notify(move |selection| {
        let Some(item) = selection.selected_item() else {
            label.set_label("Nothing selected");
            return;
        };
        let boxed = item.downcast::<BoxedAnyObject>().unwrap();
        let task: Ref<Task> = boxed.borrow();

        println!("selected: {}", task.title);
        label.set_label(&format!("Selected: {}", task.title));
    });

    // Only works because can-unselect is on and autoselect is off: with either
    // the other way round this returns without doing anything.
    let clear = gtk::Button::with_label("Clear");
    let model = selection.clone();
    clear.connect_clicked(move |_| model.set_selected(gtk::INVALID_LIST_POSITION));

    // Callable, compiles, and silently declines: SingleSelection doesn't
    // implement select_range, so the interface default returns false.
    let range = gtk::Button::with_label("Select first five");
    let model = selection.clone();
    range.connect_clicked(move |_| {
        let changed = model.select_range(0, 5, false);
        println!("SingleSelection select_range(0, 5, false) returned {changed}");
    });

    let list_view = gtk::ListView::new(Some(selection), Some(task_row::factory()));
    page_layout(&list_view, &detail, &[&clear, &range])
}

/// The list above a bar holding a status label and the page's buttons. Shared
/// with the Multi page.
pub fn page_layout(
    list_view: &gtk::ListView,
    status: &gtk::Label,
    buttons: &[&gtk::Button],
) -> gtk::Widget {
    let scrolled = gtk::ScrolledWindow::builder()
        .child(list_view)
        .vexpand(true)
        .build();

    let bar = gtk::Box::builder()
        .spacing(6)
        .margin_top(6)
        .margin_bottom(6)
        .margin_start(12)
        .margin_end(12)
        .build();
    bar.append(status);
    for button in buttons {
        bar.append(*button);
    }

    let page = gtk::Box::new(gtk::Orientation::Vertical, 0);
    page.append(&scrolled);
    page.append(&gtk::Separator::new(gtk::Orientation::Horizontal));
    page.append(&bar);
    page.upcast()
}
