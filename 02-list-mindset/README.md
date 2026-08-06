# Stop Thinking in Rows

Companion code for "Stop Thinking in Rows: GListModel and the Modern List Mindset" (unpublished draft) on [fromthearchitect.dev](https://fromthearchitect.dev).

A minimal `GListStore` + `GtkListView` + `GtkSignalListItemFactory` example.
Three tasks are shown on launch; two seconds later a fourth is appended
directly to the store, with nothing touching the view — the point of the
post.

## Build and run

```bash
cargo run
```

## Series

This is part of [A Field Guide to GTK Widgets](https://fromthearchitect.dev/posts/gtk-widgets-field-guide/) on [fromthearchitect.dev](https://fromthearchitect.dev).
