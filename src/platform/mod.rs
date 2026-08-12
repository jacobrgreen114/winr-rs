// Copyright (c) 2026 Jacob Green
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::application::ApplicationCallbacks;
use crate::input::PlatformVirtualKeyConstants;
use crate::window::{Ptr, WindowCallbacks, WindowStyle};
use crate::{Extent, Point, WingmanResult};

#[cfg(target_os = "windows")]
mod windows;

// Application

pub trait PlatformApplicationBuilder<C: ApplicationCallbacks>: Sized {
    type Application: PlatformApplication<C>;

    fn new(callbacks: C) -> Self;

    fn with_callbacks<D: ApplicationCallbacks>(self, callbacks: D) -> ApplicationBuilder<D>;

    fn build(self) -> WingmanResult<Self::Application>;
}

pub trait PlatformApplication<C: ApplicationCallbacks>: Sized {
    type Running: PlatformApplicationRunning<C>;

    fn run(self) -> Result<C::Exit, C::Error>;
}

pub trait PlatformApplicationRunning<C: ApplicationCallbacks>: Sized {
    fn shutdown(&mut self) -> WingmanResult<()>;

    fn wait_for_events(&mut self) -> WingmanResult<()>;
    fn wait_for_events_timeout(&mut self, duration: std::time::Duration) -> WingmanResult<bool>;

    fn callbacks(&self) -> &C;

    fn callbacks_mut(&mut self) -> &mut C;
}

// Window

pub trait PlatformWindowBuilder<C: WindowCallbacks>: Sized {
    type Window: PlatformWindow<C>;

    fn new(callbacks: C) -> Self;

    fn with_callbacks<D: WindowCallbacks>(self, callbacks: D) -> WindowBuilder<D>;

    fn with_title(self, title: String) -> Self;

    fn with_style(self, style: WindowStyle) -> Self;

    fn build(self) -> WingmanResult<Ptr<Self::Window>>;
}

pub trait PlatformWindow<C: WindowCallbacks>: Sized {
    fn handle(&self) -> WindowHandle;

    fn is_closed(&self) -> bool;

    fn show(&mut self) -> WingmanResult<()>;

    fn set_title(&mut self, title: String) -> WingmanResult<()>;

    fn set_pos(&mut self, pos: Point) -> WingmanResult<()>;
    fn set_size(&mut self, size: Extent) -> WingmanResult<()>;

    fn callbacks(&self) -> &C;

    fn callbacks_mut(&mut self) -> &mut C;
}

// Displays

pub trait PlatformDisplay: Sized {
    fn name(&self) -> &str;
    fn origin(&self) -> Point;
    fn extent(&self) -> Extent;
}

pub trait PlatformDisplays: Sized + Iterator<Item = Self::Display> {
    type Display: PlatformDisplay;

    fn new() -> WingmanResult<Self>;
}

// Api

pub trait PlatformWindowingApi: Sized {
    type Error;

    type ApplicationBuilder<C: ApplicationCallbacks>: PlatformApplicationBuilder<C>;

    type WindowBuilder<C: WindowCallbacks>: PlatformWindowBuilder<C>;
    type WindowHandle;

    type Displays: PlatformDisplays;

    type VirtualKeyConstants: PlatformVirtualKeyConstants;
}

pub struct WindowingApi;

pub type Error = <WindowingApi as PlatformWindowingApi>::Error;

pub type ApplicationBuilder<C> = <WindowingApi as PlatformWindowingApi>::ApplicationBuilder<C>;
pub type Application<C> = <ApplicationBuilder<C> as PlatformApplicationBuilder<C>>::Application;
pub type ApplicationRunning<C> = <Application<C> as PlatformApplication<C>>::Running;

pub type WindowBuilder<C> = <WindowingApi as PlatformWindowingApi>::WindowBuilder<C>;
pub type Window<C> = <WindowBuilder<C> as PlatformWindowBuilder<C>>::Window;
pub type WindowHandle = <WindowingApi as PlatformWindowingApi>::WindowHandle;
pub type Displays = <WindowingApi as PlatformWindowingApi>::Displays;
pub type Display = <Displays as PlatformDisplays>::Display;

pub type VirtualKeyConstants = <WindowingApi as PlatformWindowingApi>::VirtualKeyConstants;
