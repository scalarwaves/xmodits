#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(clippy::needless_lifetimes)]
#![allow(unused_imports)]


pub mod app;
mod cli;
pub mod dialog;
pub mod event;
pub mod font;
pub mod icon;
pub mod logger;
pub mod ripper;
pub mod screen;
pub mod style;
pub mod utils;
pub mod widget;

use app::XMODITS;
use cli::Mode;
use std::env;

#[cfg(all(feature = "jemallocator", not(target_env = "msvc")))]
use jemallocator::Jemalloc;

#[cfg(all(feature = "jemallocator", not(target_env = "msvc")))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

fn main() -> iced::Result {
    logger::reattach_windows_terminal();
    logger::set_panic_hook();
    logger::init_logging();

    match cli::parse(env::args().skip(1).collect()) {
        Mode::None => XMODITS::launch(),
        #[cfg(windows)]
        Mode::DragNDrop(paths) => XMODITS::launch_simple(paths),
        Mode::Version => cli::print_version(),
        Mode::Help => cli::print_help(),
        #[cfg(feature = "built")]
        Mode::BuildInfo => cli::print_info(),
        #[cfg(feature = "manual")]
        Mode::Manual => cli::print_manual(),
        Mode::Unrecognised(option) => cli::print_unrecognised(option),
    }
}
