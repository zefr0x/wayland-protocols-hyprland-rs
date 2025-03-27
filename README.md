[![crates.io](https://img.shields.io/crates/v/wayland-protocols-hyprland.svg)](https://crates.io/crates/wayland-protocols-hyprland)
[![docs.rs](https://docs.rs/wayland-protocols-hyprland/badge.svg)](https://docs.rs/wayland-protocols-hyprland)
[![release](https://github.com/zefr0x/wayland-protocols-hyprland-rs/actions/workflows/publish-release.yml/badge.svg)](https://github.com/zefr0x/wayland-protocols-hyprland-rs/actions/workflows/publish-release.yml)

# wayland-protocols-hyprland

This crate provides Wayland object definitions for the Hyprland Wayland protocol
extensions. It is meant to be used in addition to
[`wayland-client`](https://crates.io/crates/wayland-client) or
[`wayland-server`](https://crates.io/crates/wayland-server).

This crate provides bindings for the
["hyprland-protocols"](https://github.com/hyprwm/hyprland-protocols) extensions
repository.

The provided objects are controlled by the `client` and `server` cargo features,
which respectively enable the generation of client-side and server-side objects

References:

- https://github.com/Smithay/wayland-rs/tree/master/wayland-protocols-plasma
