# Filtering and Sorting Without Re-querying

Companion code for [Filtering and Sorting Without Re-querying: FilterListModel and SortListModel](https://fromthearchitect.dev/posts/gtk-widgets-filter-sort/).

One `GListStore` of `Task` objects under three decorators: a
`FilterListModel`, a `SortListModel` and a `MultiSelection`. The view only
sees the top one. Searching, hiding and re-sorting never touch the store.

- **Search** (header bar): a `StringFilter` on the `title` property.
- **Hide done:** an inverted `BoolFilter` on `done`, added to and removed
  from an `EveryFilter`.
- **Watch items:** the filter model's `watch-items`, on here (GTK's default
  is off). Turn it off, hide done tasks, then mark one done: the row stays.
- **Sort:** Title (`StringSorter`), Priority (`MultiSorter` of a
  `NumericSorter` then title) and Open first (`CustomSorter`).
- **Mark done** sets `done` on the selected tasks, then calls
  `sorter.changed()`, because `SortListModel` has no `watch-items`.
- **Add task** appends to the store. The models above put it in place.

Run it from a terminal: every `selection-changed` and `items-changed` on the
selection model is logged with what's still selected, so you can watch a
re-sort keep the selection and a filter drop it.

- `task.rs` — `Task`, now a GObject subclass with properties, since filters,
  sorters and expressions work by property name
- `task_row.rs` — the row factory, bound with expressions instead of
  bind/unbind handlers
- `filters.rs` — the search and not-done filters
- `sorters.rs` — the three sort orders
- `window.rs` — builds the model stack and the controls
- `main.rs` — app bootstrap only

## Build and run

```bash
cargo run
```

## Series

This is part of [A Field Guide to GTK Widgets](https://fromthearchitect.dev/posts/gtk-widgets-field-guide/) on [fromthearchitect.dev](https://fromthearchitect.dev).
