use std::cmp::Ordering;

use gtk::prelude::*;

use crate::task::Task;

/// The sort dropdown's entries, in the order `by_index` expects.
pub const NAMES: &[&str] = &["Title", "Priority", "Open first"];

pub fn by_index(index: u32) -> gtk::Sorter {
    match index {
        0 => title().upcast(),
        1 => priority(),
        _ => open_first(),
    }
}

fn property(name: &str) -> gtk::PropertyExpression {
    gtk::PropertyExpression::new(Task::static_type(), gtk::Expression::NONE, name)
}

/// Case-insensitive by default.
fn title() -> gtk::StringSorter {
    gtk::StringSorter::new(Some(property("title")))
}

/// A MultiSorter consults the next sorter only on a tie, so tasks of equal
/// priority come out in title order.
fn priority() -> gtk::Sorter {
    let sorter = gtk::MultiSorter::new();
    sorter.append(gtk::NumericSorter::new(Some(property("priority"))));
    sorter.append(title());
    sorter.upcast()
}

/// The closure form, for orderings an expression can't describe. Done tasks
/// sink, then title order.
fn open_first() -> gtk::Sorter {
    gtk::CustomSorter::new(|a, b| {
        let a = a.downcast_ref::<Task>().unwrap();
        let b = b.downcast_ref::<Task>().unwrap();
        a.done()
            .cmp(&b.done())
            .then_with(|| compare_titles(a, b))
            .into()
    })
    .upcast()
}

fn compare_titles(a: &Task, b: &Task) -> Ordering {
    a.title().to_lowercase().cmp(&b.title().to_lowercase())
}
