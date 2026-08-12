// Copyright (c) 2026 Jacob Green
// SPDX-License-Identifier: MIT OR Apache-2.0

// Application Builder

use crate::*;

use crate::application::*;

use crate::platform::windows::*;
use crate::platform::*;

pub struct Win32ApplicationBuilder<C: ApplicationCallbacks = ()> {
    callbacks: C,
}

impl<C: ApplicationCallbacks> PlatformApplicationBuilder<C> for Win32ApplicationBuilder<C> {
    type Application = Win32Application<C>;

    fn new(callbacks: C) -> Self {
        Self { callbacks }
    }

    fn with_callbacks<D: ApplicationCallbacks>(self, callbacks: D) -> Win32ApplicationBuilder<D> {
        Win32ApplicationBuilder { callbacks }
    }

    fn build(self) -> WingmanResult<Self::Application> {
        Ok(Self::Application {
            callbacks: self.callbacks,
        })
    }
}

// Application

pub struct Win32Application<C: ApplicationCallbacks> {
    callbacks: C,
}

impl<C: ApplicationCallbacks> PlatformApplication<C> for Win32Application<C> {
    type Running = Win32ApplicationRunning<C>;

    fn run(self) -> Result<C::Exit, C::Error> {
        let mut this = crate::application::ApplicationRunning {
            platform: Self::Running {
                callbacks: self.callbacks,
            },
        };

        C::on_startup(&mut this, &EventStartup {})?;

        while this.platform.dispatch_events() {
            C::on_events_handled(&mut this, &EventHandled {})?;
        }

        C::on_shutdown(&mut this, &EventShutdown {})
    }
}

// Application Running

pub struct Win32ApplicationRunning<C: ApplicationCallbacks> {
    callbacks: C,
}

impl<C: ApplicationCallbacks> Win32ApplicationRunning<C> {
    /// Returns `true` if out of events to dispatch, `false` if the quit msg is received.
    fn dispatch_events(&self) -> bool {
        let mut msg = win32::MSG::default();

        while unsafe { win32::PeekMessageW(&mut msg, None, 0, 0, win32::PM_REMOVE) }.as_bool() {
            if msg.message == win32::WM_QUIT {
                return false;
            }
            let _ = unsafe { win32::TranslateMessage(&msg) };
            let _result = unsafe { win32::DispatchMessageW(&msg) };
        }

        true
    }

    /// Returns `true` if an event is in queue, `false` if the wait timed out.
    fn do_wait(&self, millis: u32) -> WingmanResult<bool> {
        match unsafe { win32::MsgWaitForMultipleObjects(None, false, millis, win32::QS_ALLINPUT) } {
            win32::WAIT_FAILED => Err(win32::Error::from_thread().into()),
            win32::WAIT_TIMEOUT => Ok(false),
            _ => Ok(true),
        }
    }
}

impl<C: ApplicationCallbacks> PlatformApplicationRunning<C> for Win32ApplicationRunning<C> {
    fn shutdown(&mut self) -> WingmanResult<()> {
        unsafe { win32::PostQuitMessage(0) }
        Ok(())
    }

    fn wait_for_events(&mut self) -> WingmanResult<()> {
        unsafe { win32::WaitMessage() }.map_err(Into::into)
    }

    fn wait_for_events_timeout(&mut self, timeout: std::time::Duration) -> WingmanResult<bool> {
        const MAX_WAIT_MS: u32 = u32::MAX - 1;

        let mut millis = timeout.as_millis();
        while millis > MAX_WAIT_MS as u128 {
            if self.do_wait(MAX_WAIT_MS)? {
                return Ok(true);
            }
            millis -= MAX_WAIT_MS as u128;
        }
        self.do_wait(millis as u32)
    }

    fn callbacks(&self) -> &C {
        &self.callbacks
    }

    fn callbacks_mut(&mut self) -> &mut C {
        &mut self.callbacks
    }
}
