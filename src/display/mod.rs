// Copyright (c) 2026 Jacob Green
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::platform::PlatformDisplays;
use crate::*;
use std::fmt::Debug;

#[repr(transparent)]
pub struct Display {
    platform: platform::Display,
}

impl Display {
    pub fn name(&self) -> &str {
        self.platform.name()
    }

    pub fn origin(&self) -> Point {
        self.platform.origin()
    }

    pub fn extent(&self) -> Extent {
        self.platform.extent()
    }
}

impl Debug for Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(Display))
            .field("name", &self.name())
            .field("origin", &self.origin())
            .field("extent", &self.extent())
            .finish()
    }
}

/// Enumerates displays
#[repr(transparent)]
pub struct Displays {
    platform: platform::Displays,
}

/// Creates a `Displays` struct.
pub fn displays() -> WingmanResult<Displays> {
    platform::Displays::new().map(|platform| Displays { platform })
}

impl Iterator for Displays {
    type Item = Display;

    fn next(&mut self) -> Option<Self::Item> {
        self.platform.next().map(|platform| Display { platform })
    }
}
