// Copyright (c) 2026 Jacob Green
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::application::{ApplicationCallbacks, ApplicationRunning};
use crate::input::*;
use crate::platform::{PlatformWindow, PlatformWindowBuilder};
use crate::*;
use std::mem::transmute;
use std::ops::{Deref, DerefMut};

pub type WindowHandle = platform::WindowHandle;

pub(crate) type Ptr<T> = Box<T>;

#[derive(Debug, Copy, Clone)]
pub enum WindowStyle {
    Windowed(Windowed),
}

impl From<Windowed> for WindowStyle {
    fn from(style: Windowed) -> Self {
        Self::Windowed(style)
    }
}

impl Default for WindowStyle {
    fn default() -> Self {
        Windowed::default().into()
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Windowed {
    pub origin: Option<Point>,
    pub extent: Option<Extent>,
    pub extent_min: Option<Extent>,
    pub extent_max: Option<Extent>,
    pub resizable: bool,
    pub decorated: bool,
    // pub is_client_area_units: bool,
}

impl Windowed {
    pub fn with_origin(mut self, origin: impl Into<Option<Point>>) -> Self {
        self.origin = origin.into();
        self
    }

    pub fn with_extent(mut self, extent: impl Into<Option<Extent>>) -> Self {
        self.extent = extent.into();
        self
    }

    pub fn with_extent_min(mut self, extent: Option<Extent>) -> Self {
        self.extent_min = extent;
        self
    }

    pub fn with_extent_max(mut self, extent: Option<Extent>) -> Self {
        self.extent_max = extent;
        self
    }

    pub fn with_resizeable(mut self, resizeable: bool) -> Self {
        self.resizable = resizeable;
        self
    }

    pub fn with_decorated(mut self, decorated: bool) -> Self {
        self.decorated = decorated;
        self
    }
}

impl Default for Windowed {
    fn default() -> Self {
        Self {
            origin: None,
            extent: None,
            extent_min: None,
            extent_max: None,
            resizable: true,
            decorated: true,
            // is_client_area_units: false,
        }
    }
}

//
//  Window Events
//

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Visibility {
    Normal,
    Minimized,
    Maximized,
}

#[derive(Debug)]
pub struct EventKey {
    pub key: VirtualKey,
    pub state: KeyState,
    // pub mods: KeyModifiers,
    // pub scan_code: u32,
}

#[derive(Debug)]
pub struct EventChar {
    pub ch: char,
}

#[derive(Debug)]
pub struct EventMouseMove {
    pub position: Point,
}

#[derive(Debug)]
pub struct EventMouseButton {
    pub button: MouseButton,
    pub state: ButtonState,
    pub position: Point,
}

#[derive(Debug)]
pub struct EventMouseWheel {
    pub delta_y: i32,
    pub delta_x: i32,
    pub position: Point,
}

#[derive(Debug)]
pub struct EventShown {
    pub shown: bool,
}

#[derive(Debug)]
pub struct EventMoved {
    pub origin: Point,
}

#[derive(Debug)]
pub struct EventResized {
    pub extent: Extent,
}

#[derive(Debug)]
pub struct EventFocused {
    pub is_focused: bool,
}

#[derive(Debug)]
pub struct EventClose {}

#[derive(Debug)]
pub struct EventClosing {}

#[derive(Debug)]
pub struct EventClosed {}

#[derive(Debug)]
pub struct EventRedraw {}

#[derive(Debug)]
pub struct EventCreate {}

#[derive(Debug)]
pub struct EventVisibility {
    pub visibility: Visibility,
}

#[allow(unused_variables)]
pub trait WindowCallbacks: Sized {
    type Error: std::error::Error;

    /// Called directly after the window has been created.
    /// Returned `Err()` is currently dropped.
    fn on_created(this: &Window<Self>, event: &EventCreate) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Called when the window close event occurs.
    ///
    /// Return `true` to continue closing window or `false` to keep it open.
    /// Returned `Result` currently gets unwrapped. Therefore, `Err()` will currently cause a panic.
    fn on_close(this: &Window<Self>, event: &EventClose) -> Result<bool, Self::Error> {
        Ok(true)
    }

    /// Called directly before the window is closed.
    ///
    /// Returned `Err()` is currently dropped.
    fn on_closing(this: &Window<Self>, event: &EventClosing) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Called directly after the window is closed.
    ///
    /// Returned `Err()` is currently dropped.
    fn on_closed(this: &Window<Self>, event: &EventClosed) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Called when the platform requests the window to redraw.
    ///
    /// Returned `Err()` is currently dropped.
    fn redraw_requested(this: &Window<Self>, event: &EventRedraw) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Called when the window is shown or hidden.
    ///
    /// Returned `Err()` is currently dropped.
    fn on_shown(this: &Window<Self>, event: &EventShown) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Called when the window is minimized, maximized, or restored to normal.
    ///
    /// Returned `Err()` is currently dropped.
    fn on_visibility(this: &Window<Self>, event: &EventVisibility) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Called when the window has been moved.
    ///
    /// Returned `Err()` is currently dropped.
    fn on_moved(this: &Window<Self>, event: &EventMoved) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Called when the window has been resized.
    ///
    /// Returned `Err()` is currently dropped.
    fn on_resized(this: &Window<Self>, event: &EventResized) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Called when the window has gained or lost input focus.
    ///
    /// Returned `Err()` is currently dropped.
    fn on_focus(this: &Window<Self>, event: &EventFocused) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Called when the window has received key input.
    ///
    /// Returned `Err()` is currently dropped.
    fn on_key(this: &Window<Self>, event: &EventKey) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Called when the window has received a char input.
    ///
    /// Returned `Err()` is currently dropped.
    fn on_char(this: &Window<Self>, event: &EventChar) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Called when the mouse has moved over the window.
    ///
    /// Returned `Err()` is currently dropped.
    fn on_mouse_move(this: &Window<Self>, event: &EventMouseMove) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Called when a mouse button has been pushed.
    ///
    /// Returned `Err()` is currently dropped.
    fn on_mouse_button(this: &Window<Self>, event: &EventMouseButton) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Called when a mouse wheel has been rotated.
    ///
    /// Returned `Err()` is currently dropped.
    fn on_mouse_wheel(this: &Window<Self>, event: &EventMouseWheel) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl WindowCallbacks for () {
    type Error = WingmanError;
}

pub struct WindowBuilder<C: WindowCallbacks = ()> {
    platform: platform::WindowBuilder<C>,
}

impl WindowBuilder {
    fn new() -> Self {
        Self {
            platform: platform::WindowBuilder::new(()),
        }
    }
}

impl Default for WindowBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl<C: WindowCallbacks> WindowBuilder<C> {
    pub fn with_callbacks<D: WindowCallbacks>(self, callbacks: D) -> WindowBuilder<D> {
        WindowBuilder {
            platform: self.platform.with_callbacks(callbacks),
        }
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.platform = self.platform.with_title(title.into());
        self
    }

    pub fn with_style(mut self, style: impl Into<WindowStyle>) -> Self {
        self.platform = self.platform.with_style(style.into());
        self
    }

    pub fn build<E: From<C::Error>, A: ApplicationCallbacks<Error = E>>(
        self,
        app: &mut ApplicationRunning<A>,
    ) -> WingmanResult<WindowPtr<C>> {
        Ok(unsafe { transmute(self.platform.build()?) })
    }
}

#[repr(transparent)]
pub struct Window<C: WindowCallbacks> {
    pub(crate) platform: platform::Window<C>,
}

impl Window<()> {
    fn builder() -> WindowBuilder {
        WindowBuilder::new()
    }
}

/// shortcut for `Window::<()>::builder()`. Currently most stable way to create a `WindowBuilder`.
pub fn window() -> WindowBuilder {
    Window::builder()
}

impl<C: WindowCallbacks> Window<C> {
    pub fn handle(&self) -> WindowHandle {
        self.platform.handle()
    }

    pub fn is_closed(&self) -> bool {
        self.platform.is_closed()
    }

    pub fn show(&mut self) -> WingmanResult<()> {
        self.platform.show()
    }

    pub fn set_title(&mut self, title: impl Into<String>) -> WingmanResult<()> {
        self.platform.set_title(title.into())
    }

    pub fn set_pos(&mut self, pos: Point) -> WingmanResult<()> {
        self.platform.set_pos(pos)
    }

    pub fn set_size(&mut self, size: Extent) -> WingmanResult<()> {
        self.platform.set_size(size)
    }
}

impl<C: WindowCallbacks> Deref for Window<C> {
    type Target = C;

    fn deref(&self) -> &Self::Target {
        self.platform.callbacks()
    }
}

impl<C: WindowCallbacks> DerefMut for Window<C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.platform.callbacks_mut()
    }
}

#[repr(transparent)]
pub struct WindowPtr<C: WindowCallbacks> {
    ptr: Ptr<Window<C>>,
}

impl<C: WindowCallbacks> Deref for WindowPtr<C> {
    type Target = Window<C>;

    fn deref(&self) -> &Self::Target {
        self.ptr.as_ref()
    }
}

impl<C: WindowCallbacks> DerefMut for WindowPtr<C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.ptr.as_mut()
    }
}
