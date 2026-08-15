// Copyright (c) 2026 Jacob Green
// SPDX-License-Identifier: MIT OR Apache-2.0

#![doc = include_str!("../README.md")]
//! # Example
//! ```
#![doc = include_str!("../examples/normal_window.rs")]
//! ```

use crate::platform::PlatformDisplay;
use std::fmt::{Display, Formatter};

mod platform;

pub mod application;
pub mod display;
pub mod input;
pub mod window;

#[derive(Debug, thiserror::Error)]
pub enum WinrError {
    #[error(transparent)]
    Platform(#[from] platform::Error),
}

pub type WinrResult<T> = Result<T, WinrError>;

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
