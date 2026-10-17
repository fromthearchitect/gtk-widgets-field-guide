# Lists That Notice Their Items Changed

Companion code for the post that follows
[Filtering and Sorting Without Re-querying](https://fromthearchitect.dev/posts/gtk-widgets-filter-sort/)
(draft).

The same stack as `05-filter-sort`: a `GListStore` of `Task` objects under a
`FilterListModel`, a `SortListModel` and a `MultiSelection`. The difference is
at the bottom. The store connects a handler to each task's `title`, `done`
and `priority` notifications as the task is added, and when one changes it
emits `items_changed(pos, 1, 1)` for that task. The models above re-check
that one item, so the filter model's `watch-items` stays off and nothing
calls `sorter.changed()`.

- **Hide done:** a `CustomFilter` again. A closure filter is invisible to
  `watch-items`, but an `items-changed` reaches it like any other filter.
- **Report changes:** on by default. Turn it off, sort by Open first, then
  mark a task done: it stays where it was. Turning it back on doesn't fix
  that task. The change was never reported, and the models above only
  re-check items they're told about, so it stays out of place until it
  changes again or the sort order does.
- **Mark done** and **Raise to P1** change the selected tasks and do nothing
  else.
- **Simulate sync** completes the first open task in store order, the way a
  sync from another device or an undo would: not through the selection, not
  through a handler that knows about the sorter.
- **Add task** and **Delete** go through the store wrapper, which connects
  the handlers on the way in and disconnects them on the way out.

Run it from a terminal. The log shows each change the store reports, the
`items-changed` that reaches the view and what's still selected, every row
bind and unbind, and a line when a deleted task is freed.

- `task_store.rs`: the `GListStore` wrapper that reports item changes
- `task.rs`: `Task`, as before, plus a log line when one is freed
- `task_row.rs`: the row factory, with bind and unbind logged
- `filters.rs`: the search filter and the `CustomFilter` for done tasks
- `sorters.rs`: the three sort orders
- `window.rs`: builds the model stack and the controls
- `main.rs`: app bootstrap only

## Build and run

```bash
cargo run
```

## Series

This is part of [A Field Guide to GTK Widgets](https://fromthearchitect.dev/posts/gtk-widgets-field-guide/) on [fromthearchitect.dev](https://fromthearchitect.dev).
