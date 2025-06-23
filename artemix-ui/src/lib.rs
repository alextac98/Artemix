#![warn(clippy::all, rust_2018_idioms)]

// Main Application
mod app;
pub use app::ArtemixApp;

// Components
mod homepage;
mod plotter;