// Copyright (c) 2026 Jacob Green
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::win32::*;
use crate::display::Display;
use crate::{Extent, Point, WinrError, WinrResult};
use std::panic::{AssertUnwindSafe, catch_unwind};

#[derive(Default)]
struct Enumeration {
    displays: Vec<Display>,
    error: Option<WinrError>,
}
pub(crate) fn enumerate() -> WinrResult<Vec<Display>> {
    let mut data = Enumeration::default();
    // SAFETY: EnumDisplayMonitors invokes this callback synchronously on this thread.
    let result = unsafe {
        EnumDisplayMonitors(
            None,
            None,
            Some(monitor_proc),
            LPARAM((&mut data as *mut Enumeration) as isize),
        )
    };
    if let Some(error) = data.error {
        return Err(error);
    }
    result.ok()?;
    Ok(data.displays)
}
unsafe extern "system" fn monitor_proc(
    monitor: HMONITOR,
    _: HDC,
    _: *mut RECT,
    context: LPARAM,
) -> BOOL {
    // SAFETY: context points to enumerate()'s live, exclusively borrowed stack data.
    let data = unsafe { &mut *(context.0 as *mut Enumeration) };
    match catch_unwind(AssertUnwindSafe(|| read_monitor(monitor))) {
        Ok(Ok(display)) => {
            data.displays.push(display);
            TRUE
        }
        Ok(Err(error)) => {
            data.error = Some(error);
            FALSE
        }
        Err(_) => {
            data.error = Some(WinrError::NativeProcedurePanicked);
            FALSE
        }
    }
}
fn read_monitor(monitor: HMONITOR) -> WinrResult<Display> {
    let mut info = MONITORINFOEXW {
        monitorInfo: MONITORINFO {
            cbSize: size_of::<MONITORINFOEXW>() as u32,
            ..Default::default()
        },
        ..Default::default()
    };
    // SAFETY: info starts with MONITORINFO and cbSize describes the full allocation.
    unsafe { GetMonitorInfoW(monitor, (&mut info as *mut MONITORINFOEXW).cast()) }.ok()?;
    let end = info
        .szDevice
        .iter()
        .position(|&ch| ch == 0)
        .unwrap_or(info.szDevice.len());
    let rect = info.monitorInfo.rcMonitor;
    let width = usize::try_from(i64::from(rect.right) - i64::from(rect.left))
        .map_err(|_| WinrError::InvalidWindowOptions)?;
    let height = usize::try_from(i64::from(rect.bottom) - i64::from(rect.top))
        .map_err(|_| WinrError::InvalidWindowOptions)?;
    Ok(Display {
        name: String::from_utf16_lossy(&info.szDevice[..end]),
        origin: Point::new(rect.left as isize, rect.top as isize),
        extent: Extent::new(width, height),
        primary: info.monitorInfo.dwFlags & MONITORINFOF_PRIMARY != 0,
    })
}
