// Copyright (c) 2026 Jacob Green
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{WingmanError, WingmanResult};
use std::ops::{Deref, DerefMut};

use crate::platform::{
    PlatformApplication, PlatformApplicationBuilder, PlatformApplicationRunning,
};

pub struct EventStartup {}

pub struct EventShutdown {}

pub struct EventHandled {}

pub trait ApplicationCallbacks: Sized {
    /// Type returned from `Application::run` on main loop shutdown.
    ///
    /// `Default` constraint may be removed in the future but,
    /// is currently being used to provide a default implementation of `on_shutdown`.
    type Exit: Default;

    /// Type returned from `Application::run` on error.
    ///
    /// Can simply be `WingmanError` if your application doesn't report any other errors.
    type Error: std::error::Error + From<WingmanError>;

    /// Called directly before the main loop starts.
    ///
    /// Normally, you would create your window here.
    fn on_startup(
        this: &mut ApplicationRunning<Self>,
        event: &EventStartup,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Called after pending events have been processed.
    ///
    /// Traditionally a `ApplicationRunning::wait_for_events` variant should be called here
    /// (main loop will continuously poll for events if you don't).
    /// Alternatively, `ApplicationRunning::shutdown` can be called to notify the main loop to exit.
    fn on_events_handled(
        this: &mut ApplicationRunning<Self>,
        event: &EventHandled,
    ) -> Result<(), Self::Error> {
        this.shutdown().map_err(Into::into)
    }

    /// Called directly before the main loop exits.
    ///
    /// Returned `Result` is returned from `Application::run`.
    fn on_shutdown(
        this: &mut ApplicationRunning<Self>,
        event: &EventShutdown,
    ) -> Result<Self::Exit, Self::Error> {
        Ok(Default::default())
    }
}

impl ApplicationCallbacks for () {
    type Exit = ();
    type Error = WingmanError;
}

pub struct ApplicationBuilder<C: ApplicationCallbacks = ()> {
    platform: crate::platform::ApplicationBuilder<C>,
}

impl ApplicationBuilder {
    fn new() -> Self {
        Self {
            platform: crate::platform::ApplicationBuilder::new(Default::default()),
        }
    }
}

impl Default for ApplicationBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl<C: ApplicationCallbacks> ApplicationBuilder<C> {
    pub fn with_callbacks<D: ApplicationCallbacks>(self, callbacks: D) -> ApplicationBuilder<D> {
        ApplicationBuilder {
            platform: self.platform.with_callbacks(callbacks),
        }
    }

    pub fn build(self) -> WingmanResult<Application<C>> {
        Ok(Application {
            platform: self.platform.build()?,
        })
    }
}

pub struct Application<C: ApplicationCallbacks> {
    platform: crate::platform::Application<C>,
}

impl Application<()> {
    fn builder() -> ApplicationBuilder {
        ApplicationBuilder::new()
    }
}

/// Shortcut for `Application::<()>::builder()`. Currently most stable way to create an `ApplicationBuilder`.
pub fn application() -> ApplicationBuilder {
    ApplicationBuilder::new()
}

impl<C: ApplicationCallbacks> Application<C> {
    /// Enters the applications main loop.
    ///
    /// Standard flow pseudocode
    /// ```
    /// impl<C: ApplicationCallbacks> Application<C>
    ///     fn run(self) -> Result<C::Exit, C::Error> {
    ///         C::on_startup(...)?;
    ///         loop {
    ///             self.dispatch_pending_events(...)?;
    ///             if quit_event {
    ///                 break;
    ///             }
    ///             C::on_events_handled(...)?;
    ///         }
    ///
    ///         C::on_shutdown(...)
    ///     }
    /// }
    /// ```
    pub fn run(self) -> Result<C::Exit, C::Error> {
        self.platform.run()
    }
}

// todo - fix weird From trait issue
pub struct ApplicationRunning<C: ApplicationCallbacks> {
    pub(crate) platform: crate::platform::ApplicationRunning<C>,
}

impl<C: ApplicationCallbacks> ApplicationRunning<C> {
    pub fn shutdown(&mut self) -> WingmanResult<()> {
        self.platform.shutdown()
    }

    pub fn wait_for_events(&mut self) -> WingmanResult<()> {
        self.platform.wait_for_events()
    }

    pub fn wait_for_events_timeout(&mut self, timeout: std::time::Duration) -> WingmanResult<bool> {
        self.platform.wait_for_events_timeout(timeout)
    }
}

impl<C: ApplicationCallbacks> Deref for ApplicationRunning<C> {
    type Target = C;

    fn deref(&self) -> &Self::Target {
        self.platform.callbacks()
    }
}

impl<C: ApplicationCallbacks> DerefMut for ApplicationRunning<C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.platform.callbacks_mut()
    }
}
