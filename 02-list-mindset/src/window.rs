use adw::prelude::*;
use gtk::gio;
use gtk::glib;
use gtk::glib::BoxedAnyObject;

use crate::task::{starting_tasks, Task};
use crate::task_row;

pub fn build(app: &adw::Application) {
    let store = gio::ListStore::new::<BoxedAnyObject>();
    for task in starting_tasks() {
        store.append(&BoxedAnyObject::new(task));
    }

    // NoSelection: we're not doing anything with selection in this post —
    // that's its own post (#11). It still has to be wrapped in *a* selection
    // model, because GtkListView requires one.
    let selection = gtk::NoSelection::new(Some(store.clone()));
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
        .title("List Mindset")
        .default_width(360)
        .default_height(280)
        .content(&toolbar_view)
        .build();

    // The point of the post: two seconds after the window is already on
    // screen, append straight to the model. Nothing here touches the
    // ListView or the widget tree — the row just appears.
    glib::timeout_add_seconds_local(2, move || {
        store.append(&BoxedAnyObject::new(Task {
            title: "Ship the draft".to_string(),
            done: false,
        }));
        glib::ControlFlow::Break
    });

    window.present();
}
