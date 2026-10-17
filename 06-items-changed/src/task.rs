use gtk::glib;

mod imp {
    use std::cell::{Cell, RefCell};

    use gtk::glib;
    use gtk::glib::Properties;
    use gtk::prelude::*;
    use gtk::subclass::prelude::*;

    #[derive(Default, Properties)]
    #[properties(wrapper_type = super::Task)]
    pub struct Task {
        #[property(get, set)]
        title: RefCell<String>,
        #[property(get, set)]
        done: Cell<bool>,
        #[property(get, set)]
        priority: Cell<u32>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Task {
        const NAME: &'static str = "FieldGuideTask";
        type Type = super::Task;
    }

    #[glib::derived_properties]
    impl ObjectImpl for Task {}

    /// Logged so you can see a deleted task actually being freed, which it
    /// won't be if something still holds it.
    impl Drop for Task {
        fn drop(&mut self) {
            println!("freed {:?}", self.title.borrow());
        }
    }
}

glib::wrapper! {
    /// A GObject this time, not a BoxedAnyObject. Expressions, StringFilter
    /// and StringSorter all work by property name, so the fields have to be
    /// real GObject properties.
    pub struct Task(ObjectSubclass<imp::Task>);
}

impl Task {
    pub fn new(title: &str, done: bool, priority: u32) -> Self {
        glib::Object::builder()
            .property("title", title)
            .property("done", done)
            .property("priority", priority)
            .build()
    }
}

/// Ten tasks with mixed priorities and a few already done, so every filter and
/// sorter visibly changes the list.
pub fn starting_tasks() -> Vec<Task> {
    vec![
        Task::new("Publish the filter and sort post", true, 1),
        Task::new("Ship the button that calls the sorter", true, 2),
        Task::new("Change a task from somewhere else", false, 1),
        Task::new("Watch the list stay out of order", false, 2),
        Task::new("Have the store emit items_changed", false, 1),
        Task::new("Disconnect handlers when a task goes", false, 2),
        Task::new("Check the selection survives", false, 1),
        Task::new("Count the rebinds", false, 3),
        Task::new("Mark a batch done at once", false, 3),
        Task::new("Draft the items-changed post", false, 2),
    ]
}
