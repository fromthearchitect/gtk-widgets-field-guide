# A Field Guide to GTK Widgets — Example Code

Companion code for [A Field Guide to GTK Widgets](https://fromthearchitect.dev/posts/gtk-widgets-field-guide/) on [fromthearchitect.dev](https://fromthearchitect.dev).

Each post's example lives in its own numbered folder and is self-contained —
no shared app, no shared state between posts. Clone the folder for the post
you're reading and `cargo run` it. Examples are split into modules the way a
real app would be (data, view logic, window assembly), not dumped into one
`main.rs`, even when they're small enough to fit in one.

## Posts

| Folder | Post |
|--------|------|
| [`01-application-shell`](01-application-shell) | [The Application Shell: GtkApplicationWindow vs AdwApplicationWindow](https://fromthearchitect.dev/posts/gtk-widgets-application-shell/) |
| [`02-list-mindset`](02-list-mindset) | [Stop Thinking in Rows: GListModel and the Modern List Mindset](https://fromthearchitect.dev/posts/gtk-widgets-list-mindset/) |
| [`03-first-factory`](03-first-factory) | [Your First Factory: GtkListView and the Bind/Unbind Rhythm](https://fromthearchitect.dev/posts/gtk-widgets-first-factory/) |
| [`04-selection`](04-selection) | [Selection, Modelled: Why GTK4 Puts Selection in the Model, Not the View](https://fromthearchitect.dev/posts/gtk-widgets-selection-modelled/) |
| [`05-filter-sort`](05-filter-sort) | [Filtering and Sorting Without Re-querying: FilterListModel and SortListModel](https://fromthearchitect.dev/posts/gtk-widgets-filter-sort/) |
| [`06-items-changed`](06-items-changed) | [Keeping a GTK4 SortListModel in Order When Items Change](https://fromthearchitect.dev/posts/gtk-widgets-items-changed/) |
| [`07-columns-grids`](07-columns-grids) | [Sortable Columns in GTK4: ColumnView, ColumnViewSorter and GridView](https://fromthearchitect.dev/posts/gtk-widgets-columns-grids/) |

## Prerequisites

- Rust (stable)
- GTK4 and Libadwaita development libraries installed on your system

Or use the dev container below, which has both.

## Dev container

`.devcontainer/` holds a Fedora 44 image with the native GNOME 50 toolchain: GTK 4.22, libadwaita 1.9, rustup-managed stable Rust (with rustfmt, clippy and rust-analyzer) and lldb. One container serves every post. Each example opens in a window on your desktop.

### Prerequisites

- A Linux host running a Wayland session.
- Rootless Podman. The container maps your user onto its UID 1000 `vscode` user with `--userns=keep-id`, so your host user should be UID 1000.
- For VS Code: the Dev Containers extension, with `"dev.containers.dockerPath": "podman"` in your settings.

### First run

Open the repo root (not a post folder) in VS Code and run **Dev Containers: Reopen in Container**. The first build takes a few minutes. rust-analyzer finds each post's `Cargo.toml` by itself, so new post folders need no configuration.

Without VS Code, the Makefile runs the same image with the same flags:

```sh
make image            # build the image
make shell            # a shell in the container, with your display and session bus
make run POST=03      # build one post's example and open it on your desktop
make check            # rustfmt and clippy across every post
```

`POST` takes a folder name or any unique prefix of one.

### Running an example

From a terminal inside the container:

```sh
cd 03-first-factory
cargo run
```

Every post shares one Cargo target directory (`CARGO_TARGET_DIR`, in the `gtk-field-guide-cargo-target` volume). The GTK bindings compile once and later posts reuse them, as long as their `Cargo.lock` files resolve the same versions. Nothing is written to a `target/` inside the post folders. Binaries end up under `$CARGO_TARGET_DIR/debug/`.

GSettings uses the keyfile backend (`~/.config/glib-2.0/settings/keyfile` in the container). This keeps the examples' settings out of your host's dconf database.

### Known limitations

- **SELinux.** Containers that use the display run with `--security-opt label=disable`. Confined containers run as `container_t`, and SELinux won't let `container_t` connect to the Wayland socket (`user_tmp_t`) or the session bus (`session_dbusd_tmp_t`). No stock container type grants only that access. Relabelling the sockets with `:z` would change the labels on live host session sockets, and a custom policy module would need root to install on every host. The container still runs rootless, in a user namespace, with Podman's default capabilities and seccomp filter. It gets only the sockets listed in `devcontainer.json`, not all of `$XDG_RUNTIME_DIR`. `make check` doesn't need the display, so it runs fully confined.
- **GPU.** The container passes `/dev/dri` through, and Mesa's drivers in the image cover AMD, Intel and nouveau. With the proprietary NVIDIA driver, Mesa can't use the GPU, so GTK renders in software. You'll see `MESA-EGL: warning: egl: failed to create dri2 screen` at startup. It works, just more slowly. To use the GPU, install nvidia-container-toolkit on the host and run `sudo nvidia-ctk cdi generate --output=/etc/cdi/nvidia.yaml`. Then add `--device=nvidia.com/gpu=all` to `runArgs`, or run `make run POST=... PODMAN_FLAGS=--device=nvidia.com/gpu=all`.
- **X11.** There is no X11 fallback. GTK uses Wayland, and passing Xwayland through would also need its auth cookie, which has a random filename.

## Series

This is part of **A Field Guide to GTK Widgets** on [fromthearchitect.dev](https://fromthearchitect.dev).

## Updates

Changes made after a post was published are listed in [CHANGES.md](CHANGES.md).
