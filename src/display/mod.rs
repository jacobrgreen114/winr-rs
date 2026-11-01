// Copyright (c) 2026 Jacob Green
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Snapshots of attached monitors in physical screen coordinates.
use crate::{Extent, Point, WinrResult};

#[derive(Debug, Clone)]
pub struct Display {
    pub(crate) name: String,
    pub(crate) origin: Point,
    pub(crate) extent: Extent,
    pub(crate) primary: bool,
}
impl Display {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn origin(&self) -> Point {
        self.origin
    }
    pub fn extent(&self) -> Extent {
        self.extent
    }
    pub fn is_primary(&self) -> bool {
        self.primary
    }
}
pub fn displays() -> WinrResult<std::vec::IntoIter<Display>> {
    crate::platform::windows::display::enumerate().map(Vec::into_iter)
}
