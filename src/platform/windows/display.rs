// Copyright (c) 2026 Jacob Green
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::*;
use crate::platform::{PlatformDisplay, PlatformDisplays};
use crate::*;

pub struct Win32Monitor {
    name: String,
    origin: Point,
    extent: Extent,
}

impl PlatformDisplay for Win32Monitor {
    fn name(&self) -> &str {
        &self.name
    }

    fn origin(&self) -> Point {
        self.origin
    }

    fn extent(&self) -> Extent {
        self.extent
    }
}

pub struct Win32Monitors {
    monitors: std::vec::IntoIter<Win32Monitor>,
}

impl Iterator for Win32Monitors {
    type Item = <Self as PlatformDisplays>::Display;

    fn next(&mut self) -> Option<Self::Item> {
        self.monitors.next()
    }
}

#[derive(Default)]
struct MonitorEnumData {
    monitors: Vec<Win32Monitor>,
}

impl PlatformDisplays for Win32Monitors {
    type Display = Win32Monitor;

    fn new() -> WinrResult<Self> {
        let mut data = MonitorEnumData::default();

        unsafe {
            win32::EnumDisplayMonitors(
                None,
                None,
                Some(monitor_enum_proc),
                win32::LPARAM(core::ptr::from_mut(&mut data) as _),
            )
        }
        .ok()?;

        Ok(Self {
            monitors: data.monitors.into_iter(),
        })
    }
}

fn u16_array_to_string(bytes: &[u16]) -> String {
    let until_null = &bytes[..bytes.iter().position(|&c| c == 0).unwrap()];
    String::from_utf16(until_null).unwrap()
}

extern "system" fn monitor_enum_proc(
    hmon: win32::HMONITOR,
    _hdc: win32::HDC,
    _rect: *mut win32::RECT,
    data: win32::LPARAM,
) -> win32::BOOL {
    let data = unsafe { (data.0 as *mut MonitorEnumData).as_mut() }.unwrap();

    let mut monitor_info = win32::MONITORINFOEXW {
        monitorInfo: win32::MONITORINFO {
            cbSize: size_of::<win32::MONITORINFOEXW>().try_into().unwrap(),
            ..Default::default()
        },
        ..Default::default()
    };

    unsafe { win32::GetMonitorInfoW(hmon, &mut monitor_info as *mut _ as *mut win32::MONITORINFO) }
        .ok()
        .unwrap();

    // let mut display_device = win32::DISPLAY_DEVICEW {
    //     cb: size_of::<win32::DISPLAY_DEVICEW>().try_into().unwrap(),
    //     ..Default::default()
    // };

    // unsafe {
    //     win32::EnumDisplayDevicesW(
    //         win32::PCWSTR(monitor_info.szDevice.as_ptr()),
    //         0,
    //         &mut display_device,
    //         win32::EDD_GET_DEVICE_INTERFACE_NAME,
    //     )
    // }
    // .ok()
    // .unwrap();

    let name = u16_array_to_string(&monitor_info.szDevice);

    let origin = Point::new(
        monitor_info.monitorInfo.rcMonitor.left.try_into().unwrap(),
        monitor_info.monitorInfo.rcMonitor.top.try_into().unwrap(),
    );

    let extent = Extent::new(
        (monitor_info.monitorInfo.rcMonitor.right - monitor_info.monitorInfo.rcMonitor.left)
            .try_into()
            .unwrap(),
        (monitor_info.monitorInfo.rcMonitor.bottom - monitor_info.monitorInfo.rcMonitor.top)
            .try_into()
            .unwrap(),
    );

    // let is_primary = (monitor_info.monitorInfo.dwFlags | MONITORINFOF_PRIMARY) != 0;

    let monitor = Win32Monitor {
        name,
        origin,
        extent,
    };

    data.monitors.push(monitor);

    win32::TRUE
}
