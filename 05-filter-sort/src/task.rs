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
        Task::new("Publish the selection-model post", true, 1),
        Task::new("Wrap the store in a FilterListModel", true, 1),
        Task::new("Search titles with a StringFilter", false, 1),
        Task::new("Hide done tasks with a CustomFilter", false, 2),
        Task::new("Call changed() when the filter's state moves", false, 1),
        Task::new("Sort by title with a StringSorter", true, 3),
        Task::new("Tie-break priority with a MultiSorter", false, 2),
        Task::new("Find out what happens to the selection", false, 1),
        Task::new("Draft the filter and sort post", false, 2),
        Task::new("Push the companion code", false, 3),
    ]
}
