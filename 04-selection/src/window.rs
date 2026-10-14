use adw::prelude::*;
use gtk::gio;
use gtk::glib::BoxedAnyObject;

use crate::task::starting_tasks;
use crate::{multi, single};

pub fn build(app: &adw::Application) {
    let store = gio::ListStore::new::<BoxedAnyObject>();
    for task in starting_tasks() {
        store.append(&BoxedAnyObject::new(task));
    }

    // One store, two selection models wrapping it. Each page keeps its own
    // selection, because selection lives in the model, not in the store or
    // the view.
    let stack = adw::ViewStack::new();
    stack.add_titled(&single::page(&store), Some("single"), "Single");
    stack.add_titled(&multi::page(&store), Some("multi"), "Multi");

    let switcher = adw::ViewSwitcher::builder()
        .stack(&stack)
        .policy(adw::ViewSwitcherPolicy::Wide)
        .build();

    let header = adw::HeaderBar::new();
    header.set_title_widget(Some(&switcher));

    let toolbar_view = adw::ToolbarView::new();
    toolbar_view.add_top_bar(&header);
    toolbar_view.set_content(Some(&stack));

    let window = adw::ApplicationWindow::builder()
        .application(app)
        .title("Selection, Modelled")
        .default_width(520)
        .default_height(420)
        .content(&toolbar_view)
        .build();

    window.present();
}
