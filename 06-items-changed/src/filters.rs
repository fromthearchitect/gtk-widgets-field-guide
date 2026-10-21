use gtk::prelude::*;

use crate::task::Task;

fn property(name: &str) -> gtk::PropertyExpression {
    gtk::PropertyExpression::new(Task::static_type(), gtk::Expression::NONE, name)
}

/// Title search, described rather than coded: the StringFilter evaluates an
/// expression against each item and knows nothing about Task beyond the
/// property name. Setting its search string re-filters by itself.
pub fn search() -> gtk::StringFilter {
    let filter = gtk::StringFilter::new(Some(property("title")));
    filter.set_match_mode(gtk::StringFilterMatchMode::Substring);
    filter.set_ignore_case(true);
    filter
}

/// Keeps tasks that aren't done. Back to a CustomFilter: in the last example
/// it had to be a BoolFilter so watch-items could see the `done` property
/// through it. Here the store announces the change itself, and an
/// items-changed reaches every filter the same way, closure or not.
pub fn not_done() -> gtk::CustomFilter {
    gtk::CustomFilter::new(|item| !item.downcast_ref::<Task>().unwrap().done())
}
