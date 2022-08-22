//! # Rust Chess Library
//!
//! This is a chess library for rust.
//!
//! ## Examples
//!
//! see [`main.rs`]()

mod board;
pub use crate::board::*;

mod chess;
pub use crate::chess::*;

mod chess_gui;
pub use crate::chess_gui::*;

mod piece;
pub use crate::piece::*;

mod color;
pub use crate::color::*;

mod castle_rights;
pub use crate::castle_rights::*;

mod error;
pub use crate::error::*;

mod config;
pub use crate::config::*;

mod square;
pub use crate::square::*;

mod file;
pub use crate::file::*;

mod rank;
pub use crate::rank::*;

mod chess_move;
pub use crate::chess_move::*;

mod theme;
pub use crate::theme::*;

mod direction;
pub use crate::direction::*;

/// Run the GUI.
pub fn run(game: ChessGui) {
    let default_conf = ggez::conf::Conf {
        window_mode: ggez::conf::WindowMode::default()
            .dimensions(SCREEN_PX_SIZE.0 as f32, SCREEN_PX_SIZE.1 as f32),
        window_setup: ggez::conf::WindowSetup::default()
            .title("Chess")
            .icon("/images/icon.png"),
        backend: ggez::conf::Backend::default(),
        modules: ggez::conf::ModuleConf {
            gamepad: false,
            audio: false,
        },
    };
    let (ctx, event_loop) =
        ggez::ContextBuilder::new(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_AUTHORS"))
            .add_resource_path::<std::path::PathBuf>(
                [env!("CARGO_MANIFEST_DIR"), "resources"].iter().collect(),
            )
            .default_conf(default_conf)
            .build()
            .expect("Failed to build ggez context");

    ggez::event::run(ctx, event_loop, game)
}