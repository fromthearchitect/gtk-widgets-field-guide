# Columns and Grids

Companion code for Widgets #12, "Spreadsheets and grids", in [A Field Guide to GTK Widgets](https://fromthearchitect.dev/posts/gtk-widgets-field-guide/). The link goes here once the post is published.

One `GListStore` of `Photo` objects under a `SortListModel` and a
`MultiSelection`, shown two ways: a `ColumnView` table and a `GridView` of
tiles. Both views share the one selection model, so they share the sort order
and the selection.

- **Table:** five columns, each with its own factory and sorter. Click a
  header to sort, click again to reverse. Drag a header to reorder columns,
  or drag a column edge to resize it (Size has a fixed width and can't be
  resized).
- **Header sorting is wired, not automatic.** The `ColumnView` owns a
  `ColumnViewSorter` built from the columns' sorters, and that sorter drives
  the `SortListModel` under the selection. The view is created first, with
  no model, so the sorter exists before the model stack does.
- **Taken** and **Size** display formatted text but sort on the raw numbers
  with a `NumericSorter`. Sorted as strings, "12.4 MB" would come before
  "9.1 MB".
- **Grid:** a `GridView` on the same selection model. Select in the table,
  switch to the grid, and the same photos are selected, in the same order.
- **Rate up** raises the rating of the selected photos and does nothing
  else. `SortListModel` has no `watch-items`, so the `PhotoStore` from the
  items-changed example reports each change with `items_changed(pos, 1, 1)`,
  watching every property a column sorts on.
- **Add photo** appends through the store. Sorted by Taken, newest first (the
  starting sort), it appears at the top of both views.

Run it from a terminal: every change to the sort is logged with the sorter's
full list of sort columns. Click Size, then Camera: Camera becomes the primary
sort and Size stays behind it as the tie-breaker. The starting sort uses
`sort_by_column()`, which clears that list instead.

- `photo.rs` — `Photo`, a GObject with properties, so sorters and expressions
  can reach its fields by name
- `photo_store.rs` — the store that reports when a photo changes
- `columns.rs` — one function per column: factory, sorter, sizing
- `tile.rs` — the grid's tile factory
- `window.rs` — builds the column view, the model stack under it, and the grid
- `main.rs` — app bootstrap only

## Build and run

```bash
cargo run
```

## Series

This is part of [A Field Guide to GTK Widgets](https://fromthearchitect.dev/posts/gtk-widgets-field-guide/) on [fromthearchitect.dev](https://fromthearchitect.dev).
