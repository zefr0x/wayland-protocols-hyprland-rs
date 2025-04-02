//! This crate provides bindings to the hyprland wayland protocol extensions
//! provided in <https://github.com/hyprwm/hyprland-protocols>
//!
//! These bindings are built on top of the crates wayland-client and wayland-server.
//!
//! Each protocol module contains a `client` and a `server` submodules, for each side of the
//! protocol. The creation of these modules (and the dependency on the associated crate) is
//! controlled by the two cargo features `client` and `server`.

#![forbid(improper_ctypes, unsafe_op_in_unsafe_fn)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(rustfmt, rustfmt_skip)]

#[macro_use]
mod protocol_macro;

pub mod ctm_control {
    pub mod v1 {
        wayland_protocol!(
            "./hyprland-protocols/protocols/hyprland-ctm-control-v1.xml",
            []
        );
    }
}

pub mod focus_grab {
    pub mod v1 {
        wayland_protocol!(
            "./hyprland-protocols/protocols/hyprland-focus-grab-v1.xml",
            []
        );
    }
}

pub mod global_shortcuts {
    pub mod v1 {
        wayland_protocol!(
            "./hyprland-protocols/protocols/hyprland-global-shortcuts-v1.xml",
            []
        );
    }
}

pub mod lock_notify {
    pub mod v1 {
        wayland_protocol!(
            "./hyprland-protocols/protocols/hyprland-lock-notify-v1.xml",
            []
        );
    }
}

pub mod surface {
    pub mod v1 {
        wayland_protocol!(
            "./hyprland-protocols/protocols/hyprland-surface-v1.xml",
            []
        );
    }
}

pub mod toplevel_export {
    pub mod v1 {
        wayland_protocol!(
            "./hyprland-protocols/protocols/hyprland-toplevel-export-v1.xml",
            [wayland_protocols_wlr::foreign_toplevel::v1]
        );
    }
}

pub mod toplevel_mapping {
    pub mod v1 {
        wayland_protocol!(
            "./hyprland-protocols/protocols/hyprland-toplevel-mapping-v1.xml",
            [wayland_protocols::ext::foreign_toplevel_list::v1, wayland_protocols_wlr::foreign_toplevel::v1]
        );
    }
}
