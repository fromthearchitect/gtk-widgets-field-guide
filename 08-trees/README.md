# Trees

Companion code for Widgets #34, "Trees, at last", in [A Field Guide to GTK Widgets](https://fromthearchitect.dev/posts/gtk-widgets-field-guide/). The link goes here once the post is published.

A small photo library as a tree: years, trips inside them, photos inside
those, shown as a `ColumnView` table with an expander in the Name column
and as a `ListView` outline. One `TreeListModel` under both, one selection
model shared between them.

- **The tree model wraps a flat root.** `TreeListModel` takes the root
  `GListStore` and a closure that returns the children of an item: the
  folder's own store, or `None` for a photo. Everything the views see is a
  `TreeListRow` wrapping a Node, because the model was built with
  passthrough off. Factories and sorters reach the Node through the row's
  `item` property.
- **The closure is called more than you'd think.** Run from a terminal and
  watch `children of …?`. The expander asks whether a row *can* expand by
  calling it and throwing the answer away, and expanding a row calls it
  again every time, because collapsing drops the child model. So the Node
  owns its store and hands out the same one each time. A store built fresh
  in the closure would lose any photo added while the folder was closed.
  Returning `None` is permanent: that row is a leaf for good.
- **Sorting a tree.** The column sorters compare Nodes, as they did in the
  columns-grids example. `TreeListRowSorter` wraps the view's sorter and does
  the tree part: it walks two rows up to the level they share, compares the
  Nodes there, and keeps children under their parent. The
  `SortListModel` sits above the tree model, not below it, so every level
  sorts, not just the root.
- **Expand all** walks the tree model's rows by position and expands each;
  the children land at the next positions, so one pass opens every level.
  **Collapse all** walks backwards so each collapse only removes rows the
  loop has already passed. `autoexpand` wouldn't do: it only applies to
  rows added later.
- **Add photo** appends to the selected folder (or the selected photo's
  folder, or the Inbox) through the folder's own store. Expanded, the row
  appears in sort order. Collapsed, nothing shows until you open it, and
  the photo is there. Folder sizes update up the chain as plain property
  changes.
- **Sorting on a total that changes.** Folder sizes grow when a photo is
  added, under a sorter that may be reading them, and the sort model's
  order goes stale. Sorted by Size, the folder wouldn't move; worse, a
  photo added to a *collapsed* folder is sorted in against the live totals
  when the folder opens, and can land above its own parent. The
  items-changed trick from the last post is no help: `TreeListModel`
  treats `items_changed(pos, 1, 1)` on a child model as that row removed
  and a new one inserted, collapsed. The fix is a line: after the add,
  `sorter.changed(SorterChange::Different)` re-sorts every row in place.

- `node.rs` — `Node`, a folder or a photo, owning its children and a weak
  reference to its parent
- `library.rs` — the starting tree, and a depth-first `find`
- `columns.rs` — the Name column with its `TreeExpander`, and the plain
  Kind and Size columns
- `outline.rs` — the `ListView` row factory, an expander per row
- `window.rs` — the model stack, both views, and the three buttons
- `main.rs` — app bootstrap only

## Build and run

```bash
cargo run
```

## Series

This is part of [A Field Guide to GTK Widgets](https://fromthearchitect.dev/posts/gtk-widgets-field-guide/) on [fromthearchitect.dev](https://fromthearchitect.dev).
