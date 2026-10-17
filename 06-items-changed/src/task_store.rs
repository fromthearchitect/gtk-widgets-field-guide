use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::rc::Rc;

use gtk::gio;
use gtk::glib::SignalHandlerId;
use gtk::prelude::*;

use crate::task::Task;

/// A GListStore plus the one thing it won't do by itself: say that an item it
/// already holds has changed.
///
/// Every task that goes in gets a handler on each property the filters and
/// sorters read: `title` for the search and the Title sort, `done` for hiding
/// and Open first, `priority` for the Priority sort. When one changes, the
/// store emits items_changed(pos, 1, 1) for that task: one item out, one item
/// in, the same object both times. The filter and sort models above take that
/// as a new item and put just that one through their filter and sorter again.
///
/// Leave a property off this list and changes to it go unreported. Nothing
/// warns you; the list is just quietly out of date.
pub struct TaskStore {
    store: gio::ListStore,
    handlers: RefCell<HashMap<Task, [SignalHandlerId; 3]>>,
    reporting: Rc<Cell<bool>>,
}

impl TaskStore {
    pub fn new(tasks: &[Task]) -> Rc<Self> {
        let task_store = Rc::new(Self {
            store: gio::ListStore::new::<Task>(),
            handlers: RefCell::default(),
            reporting: Rc::new(Cell::new(true)),
        });
        for task in tasks {
            task_store.append(task);
        }
        task_store
    }

    /// What the FilterListModel wraps. Add and remove through `append` and
    /// `remove`, not on this directly, or the handlers go out of step.
    pub fn model(&self) -> &gio::ListStore {
        &self.store
    }

    /// Off: changes still happen, nobody is told. The bug from the filter and
    /// sort post comes back.
    pub fn set_reporting(&self, on: bool) {
        self.reporting.set(on);
    }

    pub fn append(&self, task: &Task) {
        let ids = [
            task.connect_title_notify(self.reporter("title")),
            task.connect_done_notify(self.reporter("done")),
            task.connect_priority_notify(self.reporter("priority")),
        ];
        self.handlers.borrow_mut().insert(task.clone(), ids);
        self.store.append(task);
    }

    /// Takes the task out and disconnects its handlers. Skip the disconnect and
    /// the store goes on hearing from a task it no longer holds.
    pub fn remove(&self, task: &Task) {
        if let Some(position) = self.store.find(task) {
            self.store.remove(position);
        }
        if let Some(ids) = self.handlers.borrow_mut().remove(task) {
            for id in ids {
                task.disconnect(id);
            }
        }
    }

    /// The handler lives on the task, so it holds the store weakly. A strong
    /// reference would be a cycle: store, task, handler, store.
    fn reporter(&self, property: &'static str) -> impl Fn(&Task) + 'static {
        let store = self.store.downgrade();
        let reporting = self.reporting.clone();
        move |task| {
            let Some(store) = store.upgrade() else {
                return;
            };
            if !reporting.get() {
                println!("{property} changed on {:?}, not reported", task.title());
                return;
            }
            match store.find(task) {
                Some(position) => {
                    println!(
                        "{property} changed on {:?}: items_changed({position}, 1, 1)",
                        task.title()
                    );
                    store.items_changed(position, 1, 1);
                }
                None => println!("{property} changed on {:?}, not in the store", task.title()),
            }
        }
    }
}

impl Drop for TaskStore {
    fn drop(&mut self) {
        for (task, ids) in self.handlers.take() {
            for id in ids {
                task.disconnect(id);
            }
        }
    }
}
