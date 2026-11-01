// Copyright (c) 2026 Jacob Green
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Thread-affine window handles and owned window events.
use crate::application::EventLoop;
use crate::input::{ButtonState, KeyState, MouseButton, VirtualKey};
use crate::platform::windows::window::NativeWindow;
use crate::{Extent, Point, WinrResult};
use std::rc::Rc;

/// Unique within an event loop; never reused during that loop's lifetime.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct WindowId(pub(crate) u64);

#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum WindowEvent {
    Created,
    /// Closing is deferred. Call `Window::close` to accept, or do nothing to veto.
    CloseRequested,
    Closed,
    RedrawRequested,
    Focused(bool),
    Shown(bool),
    /// Screen coordinates of the client area's upper-left corner.
    Moved(Point),
    /// Client area size in physical pixels (can be zero when minimized).
    Resized(Extent),
    Key {
        key: VirtualKey,
        state: KeyState,
    },
    Character(char),
    /// All mouse positions are client coordinates in physical pixels.
    MouseMoved(Point),
    MouseButton {
        button: MouseButton,
        state: ButtonState,
        position: Point,
    },
    /// Native wheel units: 120 is one conventional wheel detent.
    MouseWheel {
        delta_x: i32,
        delta_y: i32,
        position: Point,
    },
}

/// Creation options. Geometry describes the OUTER window rectangle in physical pixels.
/// Resize events describe the CLIENT area. DPI awareness is controlled by the host application.
#[derive(Debug, Clone)]
pub struct WindowBuilder {
    pub(crate) title: String,
    pub(crate) origin: Option<Point>,
    pub(crate) extent: Option<Extent>,
    pub(crate) extent_min: Option<Extent>,
    pub(crate) extent_max: Option<Extent>,
    pub(crate) resizable: bool,
    pub(crate) decorated: bool,
}
impl Default for WindowBuilder {
    fn default() -> Self {
        Self {
            title: String::new(),
            origin: None,
            extent: None,
            extent_min: None,
            extent_max: None,
            resizable: true,
            decorated: true,
        }
    }
}
impl WindowBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }
    pub fn with_origin(mut self, origin: Point) -> Self {
        self.origin = Some(origin);
        self
    }
    pub fn with_extent(mut self, extent: Extent) -> Self {
        self.extent = Some(extent);
        self
    }
    pub fn with_extent_min(mut self, extent: Extent) -> Self {
        self.extent_min = Some(extent);
        self
    }
    pub fn with_extent_max(mut self, extent: Extent) -> Self {
        self.extent_max = Some(extent);
        self
    }
    pub fn with_resizable(mut self, value: bool) -> Self {
        self.resizable = value;
        self
    }
    pub fn with_decorated(mut self, value: bool) -> Self {
        self.decorated = value;
        self
    }
    pub fn build(self, event_loop: &mut EventLoop) -> WinrResult<Window> {
        event_loop.create_window(self)
    }
}

/// A cloneable handle, usable only on its creating thread (`!Send` and `!Sync`).
/// The event loop owns native lifetime: dropping a handle does not close the window.
/// Handles may outlive the loop; operations then return `WindowClosed`.
#[derive(Clone)]
pub struct Window {
    pub(crate) native: Rc<NativeWindow>,
}
impl Window {
    pub fn id(&self) -> WindowId {
        self.native.id
    }
    pub fn is_closed(&self) -> bool {
        self.native.is_closed()
    }
    /// Borrowed native handle; never destroy it or replace its window procedure/userdata.
    pub fn handle(&self) -> WinrResult<windows::Win32::Foundation::HWND> {
        self.native.handle()
    }
    pub fn show(&self) -> WinrResult<()> {
        self.native.show()
    }
    /// Explicitly destroys the native window; already-closed windows are a no-op.
    pub fn close(&self) -> WinrResult<()> {
        self.native.close()
    }
    pub fn set_title(&self, title: impl AsRef<str>) -> WinrResult<()> {
        self.native.set_title(title.as_ref())
    }
    /// Sets the OUTER window's screen position.
    pub fn set_pos(&self, position: Point) -> WinrResult<()> {
        self.native.set_pos(position)
    }
    /// Sets the OUTER window size in physical pixels.
    pub fn set_size(&self, extent: Extent) -> WinrResult<()> {
        self.native.set_size(extent)
    }
    pub fn request_redraw(&self) -> WinrResult<()> {
        self.native.request_redraw()
    }
}
