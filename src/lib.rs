// Copyright (c) 2026 Jacob Green
// SPDX-License-Identifier: MIT OR Apache-2.0

#![doc = include_str!("../README.md")]
#![deny(unsafe_op_in_unsafe_fn)]

#[cfg(windows)]
pub mod application;
#[cfg(windows)]
pub mod display;
pub mod input;
#[cfg(windows)]
mod platform;
#[cfg(windows)]
pub mod window;

#[derive(Debug, thiserror::Error)]
pub enum WinrError {
    #[cfg(windows)]
    #[error(transparent)]
    Platform(#[from] windows::core::Error),
    #[error("invalid window geometry or title")]
    InvalidWindowOptions,
    #[error("window is closed")]
    WindowClosed,
    #[error("an event loop already exists on this thread")]
    EventLoopAlreadyExists,
    #[error("a native window procedure panicked")]
    NativeProcedurePanicked,
}
pub type WinrResult<T> = Result<T, WinrError>;

/// A position in physical pixels. Screen positions can be negative.
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}
impl Point {
    pub const fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}
impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

/// A size in physical pixels.
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Extent {
    pub width: usize,
    pub height: usize,
}
impl Extent {
    pub const fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }
    #[cfg(any(windows, test))]
    pub(crate) fn checked_i32(self) -> WinrResult<(i32, i32)> {
        let width = i32::try_from(self.width).map_err(|_| WinrError::InvalidWindowOptions)?;
        let height = i32::try_from(self.height).map_err(|_| WinrError::InvalidWindowOptions)?;
        if width == 0 || height == 0 {
            return Err(WinrError::InvalidWindowOptions);
        }
        Ok((width, height))
    }
}
impl std::fmt::Display for Extent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.width, self.height)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn geometry_rejects_zero_and_overflow() {
        assert!(Extent::new(0, 1).checked_i32().is_err());
        assert!(Extent::new(i32::MAX as usize + 1, 1).checked_i32().is_err());
        assert_eq!(Extent::new(800, 600).checked_i32().unwrap(), (800, 600));
    }
}
