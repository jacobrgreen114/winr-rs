// Copyright (c) 2026 Jacob Green
// SPDX-License-Identifier: MIT OR Apache-2.0

//! A single, statically dispatched handler receives events outside native procedures.
use crate::platform::windows::{
    win32,
    window::{NativeWindow, Shared},
};
use crate::window::{Window, WindowBuilder, WindowEvent, WindowId};
use crate::{WinrError, WinrResult};
use std::cell::Cell;
use std::rc::Rc;
use std::time::Instant;

#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum Event {
    Started,
    Window {
        id: WindowId,
        event: WindowEvent,
    },
    /// Set control flow here for animation or timed work. The default is `Wait`.
    AboutToWait,
    /// Normal exit only. Native windows are destroyed after this callback returns.
    LoopExiting,
}
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub enum ControlFlow {
    Poll,
    #[default]
    Wait,
    WaitUntil(Instant),
}
thread_local! { static ACTIVE: Cell<bool> = const { Cell::new(false) }; }

/// Owns native windows and dispatches queued events. One loop per thread.
/// Application handlers never execute from inside a Win32 window procedure.
/// Callback errors return from `run`; native windows are cleaned up on every exit path.
pub struct EventLoop {
    shared: Rc<Shared>,
    windows: Vec<Rc<NativeWindow>>,
    next_id: u64,
    control_flow: ControlFlow,
    exiting: bool,
}
impl EventLoop {
    pub fn new() -> WinrResult<Self> {
        if ACTIVE.with(|active| active.replace(true)) {
            return Err(WinrError::EventLoopAlreadyExists);
        }
        Ok(Self {
            shared: Rc::new(Shared::default()),
            windows: Vec::new(),
            next_id: 1,
            control_flow: ControlFlow::Wait,
            exiting: false,
        })
    }
    pub fn set_control_flow(&mut self, control_flow: ControlFlow) {
        self.control_flow = control_flow;
    }
    pub fn exit(&mut self) {
        self.exiting = true;
    }
    pub fn create_window(&mut self, builder: WindowBuilder) -> WinrResult<Window> {
        if self.exiting {
            return Err(WinrError::WindowClosed);
        }
        let id = WindowId(self.next_id);
        self.next_id = self
            .next_id
            .checked_add(1)
            .ok_or(WinrError::InvalidWindowOptions)?;
        let native = NativeWindow::create(id, self.shared.clone(), builder)?;
        self.windows.push(native.clone());
        Ok(Window { native })
    }
    /// Returns a handle for an open window belonging to this loop.
    pub fn window(&self, id: WindowId) -> Option<Window> {
        self.windows
            .iter()
            .find(|w| w.id == id && !w.is_closed())
            .map(|native| Window {
                native: native.clone(),
            })
    }
    pub fn run<E, F>(mut self, mut handler: F) -> Result<(), E>
    where
        E: From<WinrError>,
        F: FnMut(&mut Self, Event) -> Result<(), E>,
    {
        handler(&mut self, Event::Started)?;
        while !self.exiting {
            if self.shared.panicked.get() {
                return Err(WinrError::NativeProcedurePanicked.into());
            }
            // Bound each batch so posted traffic cannot starve application callbacks.
            for _ in 0..256 {
                let mut msg = win32::MSG::default();
                // SAFETY: msg is valid, and all processing stays on the owner thread.
                if !unsafe { win32::PeekMessageW(&mut msg, None, 0, 0, win32::PM_REMOVE) }.as_bool()
                {
                    break;
                }
                if msg.message == win32::WM_QUIT {
                    self.exiting = true;
                    break;
                }
                unsafe {
                    let _ = win32::TranslateMessage(&msg);
                    win32::DispatchMessageW(&msg);
                }
                if self.shared.panicked.get() {
                    return Err(WinrError::NativeProcedurePanicked.into());
                }
            }
            if self.exiting {
                break;
            }
            // No queue borrow survives a callback. Events produced by a handler are
            // delivered next iteration, preventing a self-generating callback from starving exit.
            let count = self.shared.events.borrow().len();
            for _ in 0..count {
                let event = self.shared.events.borrow_mut().pop_front();
                if let Some(event) = event {
                    handler(&mut self, event)?;
                }
                if self.shared.panicked.get() {
                    return Err(WinrError::NativeProcedurePanicked.into());
                }
                if self.exiting {
                    break;
                }
            }
            self.windows.retain(|w| !w.is_closed());
            if self.exiting {
                break;
            }
            handler(&mut self, Event::AboutToWait)?;
            if self.shared.panicked.get() {
                return Err(WinrError::NativeProcedurePanicked.into());
            }
            if self.exiting {
                break;
            }
            if !self.shared.events.borrow().is_empty() {
                continue;
            }
            let timeout = match self.control_flow {
                ControlFlow::Poll => continue,
                ControlFlow::Wait => u32::MAX,
                ControlFlow::WaitUntil(deadline) => {
                    // Round up to avoid spinning for the final fraction of a millisecond.
                    let duration = deadline.saturating_duration_since(Instant::now());
                    duration
                        .as_millis()
                        .saturating_add(u128::from(duration.subsec_nanos() % 1_000_000 != 0))
                        .min((u32::MAX - 1) as u128) as u32
                }
            };
            // MWMO_INPUTAVAILABLE includes messages already observed by nested native calls.
            let result = unsafe {
                win32::MsgWaitForMultipleObjectsEx(
                    None,
                    timeout,
                    win32::QS_ALLINPUT,
                    win32::MWMO_INPUTAVAILABLE,
                )
            };
            if result == win32::WAIT_FAILED {
                return Err(WinrError::from(win32::Error::from_thread()).into());
            }
        }
        if self.shared.panicked.get() {
            return Err(WinrError::NativeProcedurePanicked.into());
        }
        handler(&mut self, Event::LoopExiting)?;
        if self.shared.panicked.get() {
            return Err(WinrError::NativeProcedurePanicked.into());
        }
        Ok(())
    }
}
impl Drop for EventLoop {
    fn drop(&mut self) {
        // No registry/queue borrows cross DestroyWindow, which sends messages synchronously.
        for window in &self.windows {
            let _ = window.close();
        }
        ACTIVE.with(|active| active.set(false));
    }
}
