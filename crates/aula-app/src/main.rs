#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

//! `f87` — AULA F87 RGB lighting controller and animation suite.
//!
//! Runs the GUI by default. The Windows sign-in flag also opens the GUI.
//! Other arguments go to the CLI.

mod backend;
mod board;
mod calibration;
mod cli;
mod content;
mod editor;
mod engine;
mod icon;
mod profiles;
mod settings;
mod startup;
mod theme;
mod tray;
mod ui;
mod window_ctl;

fn effects_dir() -> std::path::PathBuf {
    if let Some(dir) = std::env::var_os("F87_EFFECTS_DIR").filter(|s| !s.is_empty()) {
        return dir.into();
    }
    if let Some(dir) = content::installed_effects_dir() {
        return dir;
    }
    // Alongside the executable when installed, or the repo's effects/ in dev.
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            let p = dir.join("effects");
            if p.is_dir() {
                return p;
            }
        }
    }
    for candidate in ["effects", "../effects", "../../effects"] {
        let p = std::path::PathBuf::from(candidate);
        if p.is_dir() {
            return p;
        }
    }
    std::path::PathBuf::from("effects")
}

fn animations_dir(effects: &std::path::Path) -> std::path::PathBuf {
    if let Some(dir) = std::env::var_os("F87_ANIMATIONS_DIR").filter(|s| !s.is_empty()) {
        return dir.into();
    }
    // Distribution packages can supply effects from a read-only location.
    // Keep user-created content in writable XDG storage in that case.
    if std::env::var_os("F87_EFFECTS_DIR").is_some_and(|s| !s.is_empty()) {
        let data = std::env::var_os("XDG_DATA_HOME")
            .filter(|s| !s.is_empty())
            .map(std::path::PathBuf::from)
            .or_else(|| {
                std::env::var_os("HOME")
                    .filter(|s| !s.is_empty())
                    .map(|h| std::path::PathBuf::from(h).join(".local/share"))
            });
        if let Some(data) = data {
            return data.join("f87/animations");
        }
    }
    effects
        .parent()
        .map(|p| p.join("animations"))
        .unwrap_or_else(|| std::path::PathBuf::from("animations"))
}

#[cfg(target_os = "windows")]
fn acquire_single_instance() -> Option<windows_sys::Win32::Foundation::HANDLE> {
    use windows_sys::Win32::{
        Foundation::{CloseHandle, GetLastError, ERROR_ALREADY_EXISTS},
        System::Threading::CreateMutexW,
        UI::WindowsAndMessaging::{
            FindWindowW, SetForegroundWindow, ShowWindow, SW_RESTORE, SW_SHOW,
        },
    };
    let mutex_name: Vec<u16> = "Local\\AULA_F87_RGB_CONTROLLER_MUTEX\0"
        .encode_utf16()
        .collect();
    let handle = unsafe { CreateMutexW(std::ptr::null(), 0, mutex_name.as_ptr()) };
    if handle.is_null() {
        return None;
    }
    if unsafe { GetLastError() } == ERROR_ALREADY_EXISTS {
        let title: Vec<u16> = "AULA F87 RGB Controller\0".encode_utf16().collect();
        let hwnd = unsafe { FindWindowW(std::ptr::null(), title.as_ptr()) };
        if !hwnd.is_null() {
            unsafe {
                ShowWindow(hwnd, SW_SHOW);
                ShowWindow(hwnd, SW_RESTORE);
                SetForegroundWindow(hwnd);
            }
        }
        unsafe { CloseHandle(handle) };
        return None;
    }
    Some(handle)
}

fn main() -> anyhow::Result<()> {
    let mut args = std::env::args_os().skip(1);
    let first_arg = args.next();
    #[cfg(target_os = "windows")]
    let sign_in_launch = first_arg.as_deref() == Some(std::ffi::OsStr::new(startup::SIGN_IN_ARG))
        && args.next().is_none();
    #[cfg(not(target_os = "windows"))]
    let sign_in_launch = false;
    if first_arg.is_some() && !sign_in_launch {
        return cli::run();
    }

    #[cfg(target_os = "windows")]
    let _single_instance_guard = match acquire_single_instance() {
        Some(h) => Some(h),
        None => {
            // Another instance is already running and has been brought to front.
            return Ok(());
        }
    };

    let dir = effects_dir();
    let settings = settings::Settings::load();
    let start_in_tray = sign_in_launch && settings.start_in_tray;
    let (rgba, w, h) = tray::icon_rgba();
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([1180.0, 780.0])
            .with_min_inner_size([760.0, 560.0])
            .with_title("AULA F87 RGB Controller")
            .with_active(!start_in_tray)
            // The taskbar/title-bar icon, same mark as the tray, instead of
            // eframe's default "e".
            .with_icon(eframe::egui::IconData {
                rgba,
                width: w,
                height: h,
            }),
        ..Default::default()
    };

    #[cfg(target_os = "linux")]
    if let Err(e) = gtk::init() {
        eprintln!("Failed to initialize GTK: {e}");
        std::process::exit(1);
    }

    eframe::run_native(
        "AULA F87 RGB Controller",
        options,
        Box::new(move |cc| Ok(Box::new(ui::App::new(cc, dir, settings, start_in_tray)))),
    )
    .map_err(|e| anyhow::anyhow!("{e}"))
}
