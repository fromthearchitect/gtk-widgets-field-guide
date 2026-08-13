use adw::prelude::*;
use gtk::gio;
use gtk::glib::BoxedAnyObject;

use crate::task::starting_tasks;
use crate::task_row;

pub fn build(app: &adw::Application) {
    let store = gio::ListStore::new::<BoxedAnyObject>();
    for task in starting_tasks() {
        store.append(&BoxedAnyObject::new(task));
    }

    // NoSelection: this post isn't about selection either — that's #11.
    let selection = gtk::NoSelection::new(Some(store));
    let list_view = gtk::ListView::new(Some(selection), Some(task_row::factory()));

    let scrolled = gtk::ScrolledWindow::builder()
        .child(&list_view)
        .vexpand(true)
        .build();

    let header = adw::HeaderBar::new();
    let toolbar_view = adw::ToolbarView::new();
    toolbar_view.add_top_bar(&header);
    toolbar_view.set_content(Some(&scrolled));

    let window = adw::ApplicationWindow::builder()
        .application(app)
        .title("First Factory")
        .default_width(360)
        .default_height(280)
        .content(&toolbar_view)
        .build();

    // The window is shorter than the task list, so scrolling recycles rows
    // straight away — the bind/unbind pairing above is what keeps the
    // dim-label class from leaking onto a task that isn't done.
    window.present();
}
