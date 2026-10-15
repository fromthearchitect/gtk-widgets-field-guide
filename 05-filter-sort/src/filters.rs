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

/// Keeps tasks that aren't done. A BoolFilter rather than a CustomFilter
/// because it's built on an expression: the FilterListModel's watch-items can
/// watch the `done` property through it. A closure is opaque, so a
/// CustomFilter never re-runs when a task changes, watch-items or not.
pub fn not_done() -> gtk::BoolFilter {
    let filter = gtk::BoolFilter::new(Some(property("done")));
    filter.set_invert(true);
    filter
}
