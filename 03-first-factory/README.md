# Your First Factory

Companion code for "Your First Factory: GtkListView and the Bind/Unbind
Rhythm" (unpublished draft) on [fromthearchitect.dev](https://fromthearchitect.dev).

A `GtkSignalListItemFactory` with all four lifecycle signals wired up.
`bind` adds a `dim-label` CSS class to done tasks; `unbind` removes it before
the row slot is recycled onto a different task, so the class never leaks
onto a task that isn't done.

- `task.rs` — the `Task` data and starting list, no GTK imports
- `task_row.rs` — the factory: `setup`, `bind`, and `unbind`, paired on purpose
- `window.rs` — builds the store, wraps it in a selection model, assembles the shell
- `main.rs` — app bootstrap only

## Build and run

```bash
cargo run
```

The window is shorter than the task list, so scrolling recycles rows
immediately — resize or scroll to see the factory in action.

## Series

This is part of [A Field Guide to GTK Widgets](https://fromthearchitect.dev/posts/gtk-widgets-field-guide/) on [fromthearchitect.dev](https://fromthearchitect.dev).
