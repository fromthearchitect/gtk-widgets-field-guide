use adw::prelude::*;
use gtk::gio;
use gtk::glib;
use gtk::glib::BoxedAnyObject;
use std::cell::Ref;

const APP_ID: &str = "dev.fromthearchitect.gtkwidgets.ListMindset";

struct Task {
    title: String,
    done: bool,
}

fn main() -> glib::ExitCode {
    let app = adw::Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &adw::Application) {
    let store = gio::ListStore::new::<BoxedAnyObject>();
    for task in starting_tasks() {
        store.append(&BoxedAnyObject::new(task));
    }

    // NoSelection: we're not doing anything with selection in this post —
    // that's its own post (#11). It still has to be wrapped in *a* selection
    // model, because GtkListView requires one.
    let selection = gtk::NoSelection::new(Some(store.clone()));

    let factory = gtk::SignalListItemFactory::new();
    factory.connect_setup(|_, list_item| {
        let label = gtk::Label::builder().xalign(0.0).build();
        list_item
            .downcast_ref::<gtk::ListItem>()
            .unwrap()
            .set_child(Some(&label));
    });
    factory.connect_bind(|_, list_item| {
        let list_item = list_item.downcast_ref::<gtk::ListItem>().unwrap();
        let boxed = list_item.item().and_downcast::<BoxedAnyObject>().unwrap();
        let task: Ref<Task> = boxed.borrow();
        let label = list_item.child().and_downcast::<gtk::Label>().unwrap();

        label.set_label(&if task.done {
            format!("✓ {}", task.title)
        } else {
            task.title.clone()
        });
    });

    let list_view = gtk::ListView::new(Some(selection), Some(factory));

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

fn starting_tasks() -> Vec<Task> {
    vec![
        Task {
            title: "Write the list-mindset post".to_string(),
            done: false,
        },
        Task {
            title: "Record the cover art".to_string(),
            done: false,
        },
        Task {
            title: "Reply to the GTK forum thread".to_string(),
            done: true,
        },
    ]
}
