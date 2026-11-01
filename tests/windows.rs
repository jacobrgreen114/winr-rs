#![cfg(windows)]

use std::cell::RefCell;
use std::rc::Rc;
use std::time::{Duration, Instant};
use windows::Win32::Foundation::{LPARAM, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::*;
use winr::application::{ControlFlow, Event, EventLoop};
use winr::input::{KeyState, VirtualKey};
use winr::window::{Window, WindowBuilder, WindowEvent};
use winr::{Extent, WinrError, WinrResult};

#[test]
fn synchronous_messages_are_queued_and_close_is_deferred() -> WinrResult<()> {
    let event_loop = EventLoop::new()?;
    let mut events = Vec::new();
    let mut idle = 0;
    let mut handle = None;
    let mut id = None;
    event_loop.run(|event_loop, event| -> WinrResult<()> {
        match event {
            Event::Started => {
                event_loop.set_control_flow(ControlFlow::Poll);
                let window = WindowBuilder::new()
                    .with_extent(Extent::new(320, 240))
                    .build(event_loop)?;
                id = Some(window.id());
                // Each call sends nested native messages while Window remains shared-borrowed.
                window.set_size(Extent::new(400, 300))?;
                window.set_pos(winr::Point::new(-100, 50))?;
                let hwnd = window.handle()?;
                unsafe {
                    SendMessageW(hwnd, WM_CHAR, Some(WPARAM(0xd83d)), Some(LPARAM(0)));
                    SendMessageW(hwnd, WM_CHAR, Some(WPARAM(0xde00)), Some(LPARAM(0)));
                    SendMessageW(hwnd, WM_KEYDOWN, Some(WPARAM(0xff)), Some(LPARAM(0)));
                    SendMessageW(
                        hwnd,
                        WM_KEYDOWN,
                        Some(WPARAM(0x10)),
                        Some(LPARAM(0x36 << 16)),
                    );
                    SendMessageW(hwnd, WM_CLOSE, None, None);
                }
                assert!(
                    !window.is_closed(),
                    "WM_CLOSE must wait for the application decision"
                );
                assert!(
                    events.is_empty(),
                    "native calls must not invoke the handler"
                );
                handle = Some(window);
            }
            Event::Window {
                id: event_id,
                event,
            } => {
                assert_eq!(Some(event_id), id);
                if event == WindowEvent::CloseRequested {
                    let window = handle.as_ref().unwrap();
                    window.close()?;
                    window.close()?; // idempotent, including nested destruction
                    assert!(window.is_closed());
                    assert!(matches!(
                        window.set_title("closed"),
                        Err(WinrError::WindowClosed)
                    ));
                }
                if event == WindowEvent::Closed {
                    event_loop.exit();
                }
                events.push(event);
            }
            Event::AboutToWait => {
                idle += 1;
                assert!(idle < 100, "expected Closed event was not delivered");
            }
            _ => {}
        }
        Ok(())
    })?;
    assert_eq!(events.first(), Some(&WindowEvent::Created));
    assert_eq!(
        events.iter().filter(|e| **e == WindowEvent::Closed).count(),
        1
    );
    assert!(events.contains(&WindowEvent::Character('😀')));
    assert!(events.contains(&WindowEvent::Key {
        key: VirtualKey::Unknown(0xff),
        state: KeyState::Press
    }));
    assert!(events.contains(&WindowEvent::Key {
        key: VirtualKey::RightShift,
        state: KeyState::Press
    }));
    Ok(())
}

#[test]
fn registry_owns_windows_and_loop_teardown_invalidates_surviving_handles() -> WinrResult<()> {
    let mut event_loop = EventLoop::new()?;
    assert!(matches!(
        EventLoop::new(),
        Err(WinrError::EventLoopAlreadyExists)
    ));
    let window = WindowBuilder::new().build(&mut event_loop)?;
    let id = window.id();
    drop(window);
    let survivor = event_loop
        .window(id)
        .expect("dropping a handle must not destroy native state");
    assert!(!survivor.is_closed());
    drop(event_loop);
    assert!(survivor.is_closed());
    assert!(matches!(survivor.show(), Err(WinrError::WindowClosed)));
    drop(EventLoop::new()?); // thread-local ownership guard must have been released
    Ok(())
}

#[derive(Debug)]
enum AppError {
    Native(WinrError),
    Callback,
}
impl From<WinrError> for AppError {
    fn from(e: WinrError) -> Self {
        Self::Native(e)
    }
}

#[test]
fn callback_errors_return_and_destroy_windows() -> WinrResult<()> {
    let survivor: Rc<RefCell<Option<Window>>> = Rc::new(RefCell::new(None));
    let result = EventLoop::new()?.run(|event_loop, event| -> Result<(), AppError> {
        if event == Event::Started {
            *survivor.borrow_mut() = Some(WindowBuilder::new().build(event_loop)?);
        }
        if matches!(
            event,
            Event::Window {
                event: WindowEvent::Created,
                ..
            }
        ) {
            return Err(AppError::Callback);
        }
        Ok(())
    });
    match result {
        Err(AppError::Callback) => {}
        Err(AppError::Native(error)) => panic!("unexpected native error: {error}"),
        Ok(()) => panic!("callback error was lost"),
    }
    assert!(survivor.borrow().as_ref().unwrap().is_closed());
    Ok(())
}

#[test]
fn callback_panics_unwind_in_rust_and_cleanup_still_runs() -> WinrResult<()> {
    let mut survivor = None;
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        EventLoop::new()
            .unwrap()
            .run(|event_loop, event| -> WinrResult<()> {
                if event == Event::Started {
                    survivor = Some(WindowBuilder::new().build(event_loop)?);
                    panic!("application panic");
                }
                Ok(())
            })
    }));
    assert!(result.is_err());
    assert!(survivor.as_ref().unwrap().is_closed());
    drop(EventLoop::new()?);
    Ok(())
}

#[test]
fn invalid_options_are_errors_and_do_not_poison_the_loop() -> WinrResult<()> {
    let mut event_loop = EventLoop::new()?;
    for builder in [
        WindowBuilder::new().with_title("invalid\0title"),
        WindowBuilder::new().with_extent(Extent::new(0, 10)),
        WindowBuilder::new().with_extent(Extent::new(usize::MAX, 1)),
        WindowBuilder::new()
            .with_extent_min(Extent::new(300, 300))
            .with_extent_max(Extent::new(100, 100)),
    ] {
        assert!(matches!(
            builder.build(&mut event_loop),
            Err(WinrError::InvalidWindowOptions)
        ));
    }
    let window = WindowBuilder::new().build(&mut event_loop)?;
    window.close()?;
    Ok(())
}

#[test]
fn wait_until_wakes_without_messages() -> WinrResult<()> {
    let deadline = Instant::now() + Duration::from_millis(25);
    let mut reached = false;
    EventLoop::new()?.run(|event_loop, event| -> WinrResult<()> {
        match event {
            Event::Started => event_loop.set_control_flow(ControlFlow::WaitUntil(deadline)),
            Event::AboutToWait if Instant::now() >= deadline => {
                reached = true;
                event_loop.exit();
            }
            _ => {}
        }
        Ok(())
    })?;
    assert!(reached);
    Ok(())
}

#[test]
fn close_veto_and_multiple_windows_have_independent_lifetimes() -> WinrResult<()> {
    let mut event_loop = EventLoop::new()?;
    let first = WindowBuilder::new().build(&mut event_loop)?;
    let second = WindowBuilder::new().build(&mut event_loop)?;
    unsafe {
        SendMessageW(first.handle()?, WM_CLOSE, None, None);
    }
    let mut vetoed = false;
    let mut idle = 0;
    event_loop.run(|event_loop, event| -> WinrResult<()> {
        event_loop.set_control_flow(ControlFlow::Poll);
        match event {
            Event::Window {
                id,
                event: WindowEvent::CloseRequested,
            } if id == first.id() => {
                vetoed = true; // no close() means veto
                assert!(!first.is_closed());
                assert!(!second.is_closed());
                second.close()?;
            }
            Event::Window {
                id,
                event: WindowEvent::Closed,
            } if id == second.id() => {
                assert!(!first.is_closed());
                assert!(event_loop.window(first.id()).is_some());
                event_loop.exit();
            }
            Event::AboutToWait => {
                idle += 1;
                assert!(idle < 100);
            }
            _ => {}
        }
        Ok(())
    })?;
    assert!(vetoed);
    assert!(first.is_closed()); // remaining window cleaned up on loop exit
    assert!(second.is_closed());
    Ok(())
}
