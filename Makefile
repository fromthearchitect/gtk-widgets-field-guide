# The dev container without VS Code: the same image and flags as
# .devcontainer/devcontainer.json, for anyone not using VS Code.
#
#   make image               build the image (run again after changing the Containerfile)
#   make shell               interactive shell, with the host display and session bus
#   make run POST=03         build one post's example and open it on the host desktop
#   make check               rustfmt and clippy across every post
#
# POST is a folder name or any unique prefix of one: 03, 03-first-factory.
#
# Keep the volume names, mount paths and GUI flags in step with devcontainer.json.

IMAGE     ?= gtk-field-guide-dev
# Extra `podman run` flags, e.g. PODMAN_FLAGS=--device=nvidia.com/gpu=all for
# the proprietary NVIDIA driver (see README.md, "Known limitations").
PODMAN_FLAGS ?=
WORKSPACE := /workspaces/$(notdir $(CURDIR))
# -t only when there is a terminal, so the targets also work in CI.
TTY       := $(shell [ -t 0 ] && echo -t)

# Every numbered folder with a Cargo.toml is a post.
POSTS     := $(patsubst %/Cargo.toml,%,$(wildcard [0-9]*/Cargo.toml))
POST_DIR   = $(filter $(POST)%,$(POSTS))

# keep-id maps the host user onto the image's UID 1000 vscode user; z relabels
# the checkout so the confined container can write to it; the volumes keep
# downloaded crates and the shared target dir across image rebuilds.
PODMAN_RUN := podman run --rm -i $(TTY) \
	--userns=keep-id --user vscode \
	-v "$(CURDIR):$(WORKSPACE):z" -w "$(WORKSPACE)" \
	-v gtk-field-guide-cargo-registry:/home/vscode/.cargo/registry \
	-v gtk-field-guide-cargo-git:/home/vscode/.cargo/git \
	-v gtk-field-guide-cargo-target:/home/vscode/.cache/cargo-target \
	-v gtk-field-guide-gh:/home/vscode/.config/gh \
	$(PODMAN_FLAGS)

# The host session: Wayland socket, session bus and accessibility bus mounted at
# their host paths (so addresses handed out over D-Bus, like the a11y bus's, are
# valid inside too), and /dev/dri for GPU rendering. Only individual sockets, not
# all of $(XDG_RUNTIME_DIR), which also holds keyring, gpg and ssh-agent sockets.
#
# label=disable because the confined container_t domain may not connect to the
# compositor or the bus, and no stock container type allows only that. See the
# README's "Known limitations" for the alternatives and why they're worse.
XDG_RUNTIME_DIR ?= /run/user/$(shell id -u)
WAYLAND_DISPLAY ?= wayland-0
GUI_FLAGS := \
	--security-opt label=disable \
	--device /dev/dri \
	-e XDG_RUNTIME_DIR=$(XDG_RUNTIME_DIR) \
	-e WAYLAND_DISPLAY=$(WAYLAND_DISPLAY) \
	-e DBUS_SESSION_BUS_ADDRESS=unix:path=$(XDG_RUNTIME_DIR)/bus \
	-v $(XDG_RUNTIME_DIR)/$(WAYLAND_DISPLAY):$(XDG_RUNTIME_DIR)/$(WAYLAND_DISPLAY) \
	-v $(XDG_RUNTIME_DIR)/bus:$(XDG_RUNTIME_DIR)/bus \
	-v $(XDG_RUNTIME_DIR)/at-spi:$(XDG_RUNTIME_DIR)/at-spi

.PHONY: image shell run check

image:
	podman build -t $(IMAGE) -f .devcontainer/Containerfile .devcontainer

shell:
	$(PODMAN_RUN) $(GUI_FLAGS) $(IMAGE) bash

run:
	@[ -n "$(POST)" ] || { echo 'usage: make run POST=<folder or prefix>; posts: $(POSTS)' >&2; exit 2; }
	@[ $(words $(POST_DIR)) -eq 1 ] || { echo 'POST=$(POST) matches [$(POST_DIR)]; posts: $(POSTS)' >&2; exit 2; }
	$(PODMAN_RUN) $(GUI_FLAGS) -w "$(WORKSPACE)/$(POST_DIR)" $(IMAGE) cargo run

# check doesn't need the host session, so it runs fully confined.
check:
	$(PODMAN_RUN) $(IMAGE) bash -c 'set -e; for p in $(POSTS); do echo "== $$p"; (cd "$$p" && cargo fmt --check && cargo clippy --all-targets); done'
