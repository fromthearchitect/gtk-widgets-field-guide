# Selection, Modelled

Companion code for [Selection, Modelled: Why GTK4 Puts Selection in the Model, Not the View](https://fromthearchitect.dev/posts/gtk-widgets-selection-modelled/).

One `GListStore` wrapped by two selection models, one per tab. Each keeps its
own selection, because selection lives in the model, not in the view or the
store.

- **Single:** a `SingleSelection` with `autoselect` off and `can-unselect`
  on, set *before* the model so the list starts empty. A detail label follows
  `selected-item`. **Clear** sets `INVALID_LIST_POSITION`, and **Select first
  five** shows `select_range` silently declining (it prints `false`).
- **Multi:** a `MultiSelection`. On every `selection-changed`, it logs the
  signal's `(position, n_items)` next to the real selection size, re-queried
  from the bitset, so you can watch the two disagree.

Run it from a terminal to see the log on stdout.

- `task.rs` — the `Task` data and starting list (ten items), no GTK imports
- `task_row.rs` — the row factory, unchanged from `03-first-factory`
- `single.rs` — the SingleSelection page, plus the page layout both tabs share
- `multi.rs` — the MultiSelection page and the bitset iteration
- `window.rs` — builds the store, puts both pages in an `AdwViewStack`
- `main.rs` — app bootstrap only

## Build and run

```bash
cargo run
```

Select row 3, then ctrl-click row 7 on the Multi tab, then press
**Select all**:

```text
args = (3, 1),  selection size = 1
args = (7, 1),  selection size = 2
args = (0, 10),  selection size = 10
```

## Series

This is part of [A Field Guide to GTK Widgets](https://fromthearchitect.dev/posts/gtk-widgets-field-guide/) on [fromthearchitect.dev](https://fromthearchitect.dev).
