// Copyright (c) 2026 Jacob Green
// SPDX-License-Identifier: MIT OR Apache-2.0

//! # winr
//!
//! `winr` is a cross-platform windowing framework.
//!
//! ### State
//! `winr` is currently in early days of maturity with high levels of refactoring being expected.
//! Ideas and input for the future of `winr` is welcomed as it grows.
//!
//! ### Aspiration
//! The aspiration of `winr` is to be a very fast and lightweight windowing framework.
//! Dynamic dispatch usage is kept to a minimum except where absolutely necessary.
//!
//! ### Platforms
//! Currently only Windows is supported.
//! MacOS and Wayland implementations are mostly implemented and will be released soon.
//!

use crate::platform::PlatformDisplay;
use std::fmt::{Display, Formatter};

mod platform;

pub mod application;
pub mod display;
pub mod input;
pub mod window;

#[derive(Debug, thiserror::Error)]
pub enum WingmanError {
    #[error(transparent)]
    Platform(#[from] platform::Error),
}

pub type WingmanResult<T> = Result<T, WingmanError>;

#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    pub const fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({}, {})", self.x, self.y))
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Extent {
    pub width: usize,
    pub height: usize,
}

impl Extent {
    pub const fn new(x: usize, y: usize) -> Self {
        Self {
            width: x,
            height: y,
        }
    }
}

impl Display for Extent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({}, {})", self.width, self.height))
    }
}
