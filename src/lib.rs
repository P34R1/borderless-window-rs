use std::{collections::HashMap, ffi::OsString, os::windows::ffi::OsStringExt};

use winapi::{
    shared::{minwindef, windef::HWND},
    um::winuser::{
        self, GWL_STYLE, HWND_TOP, MONITOR_DEFAULTTONEAREST, SWP_FRAMECHANGED, SWP_SHOWWINDOW,
        SW_MAXIMIZE, WS_POPUP,
    },
};

/// # Safety
///
/// HWND should be a valid pointer to a program.
pub unsafe fn set_borderless_fullscreen(hwnd: HWND) {
    let mut rect = std::mem::zeroed();
    unsafe { winuser::GetClientRect(hwnd, &mut rect) };

    let monitor = unsafe { winuser::MonitorFromWindow(hwnd, MONITOR_DEFAULTTONEAREST) };
    let mut monitor_info: winuser::MONITORINFO = unsafe { std::mem::zeroed() };
    monitor_info.cbSize = std::mem::size_of::<winuser::MONITORINFO>() as u32;
    unsafe { winuser::GetMonitorInfoW(monitor, &mut monitor_info) };

    let monitor_width = monitor_info.rcMonitor.right - monitor_info.rcMonitor.left;
    let monitor_height = monitor_info.rcMonitor.bottom - monitor_info.rcMonitor.top;

    // Set the window style to borderless
    let style = WS_POPUP;
    unsafe { winuser::SetWindowLongPtrW(hwnd, GWL_STYLE, style as _) };

    // Set window position and size to cover the entire screen
    unsafe {
        winuser::SetWindowPos(
            hwnd,
            HWND_TOP,
            monitor_info.rcMonitor.left,
            monitor_info.rcMonitor.top,
            monitor_width,
            monitor_height,
            SWP_FRAMECHANGED | SWP_SHOWWINDOW,
        )
    };

    // Maximize the window
    unsafe { winuser::ShowWindow(hwnd, SW_MAXIMIZE) };
}

extern "system" fn enum_windows_proc(hwnd: HWND, l_param: minwindef::LPARAM) -> minwindef::BOOL {
    let mut buffer = [0; 512];
    let end_of_str =
        unsafe { winuser::GetWindowTextW(hwnd, buffer.as_mut_ptr(), buffer.len() as i32) } as usize;

    let window_title = OsString::from_wide(&buffer[..end_of_str])
        .into_string()
        .unwrap_or_default();

    if !window_title.is_empty() {
        let windows = unsafe { &mut *(l_param as *mut HashMap<String, HWND>) };
        windows.insert(window_title, hwnd);
    }

    // Succeded
    minwindef::TRUE
}

pub fn read_all_windows() -> HashMap<String, HWND> {
    let mut windows: HashMap<String, HWND> = HashMap::new();

    let windows_raw_ptr = &mut windows as *mut HashMap<String, HWND> as minwindef::LPARAM;
    unsafe { winuser::EnumWindows(Some(enum_windows_proc), windows_raw_ptr) };

    windows
}
