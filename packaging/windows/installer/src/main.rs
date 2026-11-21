#![windows_subsystem = "windows"]

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const APP_NAME: &str = "AULA F87";
const APP_TITLE: &str = "AULA F87 RGB Controller";
const VERSION: &str = "0.5.2";

// Embedded release assets
const F87_EXE: &[u8] = include_bytes!("../../../../target/release/f87.exe");
const EFFECT_BREATHE: &[u8] = include_bytes!("../../../../effects/breathe.rhai");
const EFFECT_RIPPLE: &[u8] = include_bytes!("../../../../effects/ripple.rhai");
const EFFECT_SPIRAL: &[u8] = include_bytes!("../../../../effects/spiral.rhai");
const EFFECT_AURORA: &[u8] = include_bytes!("../../../../effects/aurora.rhai");
const EFFECT_FIRE: &[u8] = include_bytes!("../../../../effects/fire.rhai");
const EFFECT_MATRIX_RAIN: &[u8] = include_bytes!("../../../../effects/matrix_rain.rhai");
const EFFECT_CYBER_PULSE: &[u8] = include_bytes!("../../../../effects/cyber_pulse.rhai");
const EFFECT_MUSIC_EQUALIZER: &[u8] = include_bytes!("../../../../effects/music_equalizer.rhai");
const ANIM_CYBER_WAVE: &[u8] = include_bytes!("../../../../animations/f87_cyber_wave.klx");
const ANIM_GAMING_ZONES: &[u8] = include_bytes!("../../../../animations/f87_gaming_zones.klx");
const ANIM_RAINBOW_SWEEP: &[u8] = include_bytes!("../../../../animations/f87_rainbow_sweep.klx");
const LICENSE_MIT: &[u8] = include_bytes!("../../../../LICENSE-MIT");
const LICENSE_APACHE: &[u8] = include_bytes!("../../../../LICENSE-APACHE");
const README: &[u8] = include_bytes!("../../../../README.md");

const UNINSTALL_BAT: &str = r#"@echo off
taskkill /F /IM f87.exe >nul 2>&1
powershell -NoProfile -NonInteractive -Command "$d = [System.Environment]::GetFolderPath('Desktop'); $p = [System.Environment]::GetFolderPath('Programs'); Remove-Item (Join-Path $d 'AULA F87.lnk') -Force -ErrorAction SilentlyContinue; Remove-Item (Join-Path $p 'AULA F87.lnk') -Force -ErrorAction SilentlyContinue"
reg delete "HKCU\Software\Microsoft\Windows\CurrentVersion\Uninstall\AULA-F87" /f >nul 2>&1
start /b "" cmd /c "timeout /t 1 /nobreak >nul & rd /s /q \"%~dp0\""
"#;

#[cfg(windows)]
fn show_message_box(text: &str, caption: &str, u_type: u32) -> i32 {
    use std::os::windows::ffi::OsStrExt;
    let wide_text: Vec<u16> = std::ffi::OsStr::new(text)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();
    let wide_caption: Vec<u16> = std::ffi::OsStr::new(caption)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();

    extern "system" {
        fn MessageBoxW(
            hwnd: *mut std::ffi::c_void,
            lpText: *const u16,
            lpCaption: *const u16,
            uType: u32,
        ) -> i32;
    }

    unsafe { MessageBoxW(std::ptr::null_mut(), wide_text.as_ptr(), wide_caption.as_ptr(), u_type) }
}

const MB_OK: u32 = 0x00000000;
const MB_YESNO: u32 = 0x00000004;
const MB_ICONINFORMATION: u32 = 0x00000040;
const MB_ICONERROR: u32 = 0x00000010;
const IDYES: i32 = 6;

fn to_base64(bytes: &[u8]) -> String {
    const B64_CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut res = String::with_capacity((bytes.len() + 2) / 3 * 4);
    for chunk in bytes.chunks(3) {
        let b0 = chunk[0];
        let b1 = if chunk.len() > 1 { chunk[1] } else { 0 };
        let b2 = if chunk.len() > 2 { chunk[2] } else { 0 };
        let n = ((b0 as u32) << 16) | ((b1 as u32) << 8) | (b2 as u32);
        res.push(B64_CHARS[((n >> 18) & 63) as usize] as char);
        res.push(B64_CHARS[((n >> 12) & 63) as usize] as char);
        if chunk.len() > 1 {
            res.push(B64_CHARS[((n >> 6) & 63) as usize] as char);
        } else {
            res.push('=');
        }
        if chunk.len() > 2 {
            res.push(B64_CHARS[(n & 63) as usize] as char);
        } else {
            res.push('=');
        }
    }
    res
}

fn run_powershell_script(script: &str) -> Result<(), String> {
    let utf16_bytes: Vec<u8> = script
        .encode_utf16()
        .flat_map(|u| u.to_le_bytes())
        .collect();
    let b64 = to_base64(&utf16_bytes);

    let output = Command::new("powershell")
        .args([
            "-NoProfile",
            "-NonInteractive",
            "-ExecutionPolicy",
            "Bypass",
            "-EncodedCommand",
            &b64,
        ])
        .output()
        .map_err(|e| format!("Failed to execute powershell: {}", e))?;

    if !output.status.success() {
        let err = String::from_utf8_lossy(&output.stderr);
        return Err(format!("PowerShell execution failed: {}", err));
    }
    Ok(())
}

fn default_install_dir() -> PathBuf {
    if let Ok(local_app_data) = env::var("LOCALAPPDATA") {
        PathBuf::from(local_app_data).join("Programs").join("AULA-F87")
    } else if let Ok(user_profile) = env::var("USERPROFILE") {
        PathBuf::from(user_profile)
            .join("AppData")
            .join("Local")
            .join("Programs")
            .join("AULA-F87")
    } else {
        PathBuf::from(r"C:\Program Files\AULA-F87")
    }
}

fn create_shortcuts(target_exe: &Path, target_dir: &Path) -> Result<(), String> {
    let ps_script = format!(
        r#"$ws = New-Object -ComObject WScript.Shell
$desktop = [System.Environment]::GetFolderPath('Desktop')
if ($desktop -and (Test-Path $desktop)) {{
    $s1 = $ws.CreateShortcut((Join-Path $desktop 'AULA F87.lnk'))
    $s1.TargetPath = '{exe}'
    $s1.WorkingDirectory = '{dir}'
    $s1.Description = '{desc}'
    $s1.Save()
}}
$programs = [System.Environment]::GetFolderPath('Programs')
if ($programs -and (Test-Path $programs)) {{
    $s2 = $ws.CreateShortcut((Join-Path $programs 'AULA F87.lnk'))
    $s2.TargetPath = '{exe}'
    $s2.WorkingDirectory = '{dir}'
    $s2.Description = '{desc}'
    $s2.Save()
}}
"#,
        exe = target_exe.display(),
        dir = target_dir.display(),
        desc = APP_TITLE,
    );

    run_powershell_script(&ps_script)
}

fn remove_shortcuts() {
    let ps_script = r#"$desktop = [System.Environment]::GetFolderPath('Desktop')
if ($desktop) {
    $s1 = Join-Path $desktop 'AULA F87.lnk'
    if (Test-Path $s1) { Remove-Item $s1 -Force -ErrorAction SilentlyContinue }
}
$programs = [System.Environment]::GetFolderPath('Programs')
if ($programs) {
    $s2 = Join-Path $programs 'AULA F87.lnk'
    if (Test-Path $s2) { Remove-Item $s2 -Force -ErrorAction SilentlyContinue }
}
"#;

    let _ = run_powershell_script(ps_script);
}

fn register_uninstall(install_dir: &Path, exe_path: &Path, uninstall_bat_path: &Path) {
    let key = r"HKCU\Software\Microsoft\Windows\CurrentVersion\Uninstall\AULA-F87";
    let _ = Command::new("reg")
        .args(["add", key, "/v", "DisplayName", "/d", APP_TITLE, "/f"])
        .output();
    let _ = Command::new("reg")
        .args(["add", key, "/v", "DisplayVersion", "/d", VERSION, "/f"])
        .output();
    let _ = Command::new("reg")
        .args(["add", key, "/v", "Publisher", "/d", "AULA", "/f"])
        .output();
    let _ = Command::new("reg")
        .args([
            "add",
            key,
            "/v",
            "InstallLocation",
            "/d",
            install_dir.to_str().unwrap_or_default(),
            "/f",
        ])
        .output();
    let _ = Command::new("reg")
        .args([
            "add",
            key,
            "/v",
            "DisplayIcon",
            "/d",
            &format!("{},0", exe_path.display()),
            "/f",
        ])
        .output();
    let _ = Command::new("reg")
        .args([
            "add",
            key,
            "/v",
            "UninstallString",
            "/d",
            &format!("\"{}\"", uninstall_bat_path.display()),
            "/f",
        ])
        .output();
}

fn unregister_uninstall() {
    let key = r"HKCU\Software\Microsoft\Windows\CurrentVersion\Uninstall\AULA-F87";
    let _ = Command::new("reg")
        .args(["delete", key, "/f"])
        .output();
}

fn terminate_existing_app() {
    let _ = Command::new("taskkill")
        .args(["/F", "/IM", "f87.exe"])
        .output();
}

fn uninstall(install_dir: &Path, silent: bool) -> Result<(), String> {
    terminate_existing_app();
    remove_shortcuts();
    unregister_uninstall();

    if install_dir.exists() {
        fs::remove_dir_all(install_dir)
            .map_err(|e| format!("Failed to remove directory {}: {}", install_dir.display(), e))?;
    }

    if !silent {
        show_message_box(
            "AULA F87 has been uninstalled successfully.",
            APP_TITLE,
            MB_OK | MB_ICONINFORMATION,
        );
    }
    Ok(())
}

fn install(install_dir: &Path, silent: bool, launch_flag: bool, no_launch: bool) -> Result<(), String> {
    // Terminate existing process if running to allow file overwrite
    terminate_existing_app();

    // Create target directories
    let effects_dir = install_dir.join("effects");
    fs::create_dir_all(&effects_dir)
        .map_err(|e| format!("Failed to create directory {}: {}", effects_dir.display(), e))?;

    // Write application executable
    let exe_path = install_dir.join("f87.exe");
    fs::write(&exe_path, F87_EXE)
        .map_err(|e| format!("Failed to write {}: {}", exe_path.display(), e))?;

    // Write lighting effects
    fs::write(effects_dir.join("breathe.rhai"), EFFECT_BREATHE)
        .map_err(|e| format!("Failed to write breathe.rhai: {}", e))?;
    fs::write(effects_dir.join("ripple.rhai"), EFFECT_RIPPLE)
        .map_err(|e| format!("Failed to write ripple.rhai: {}", e))?;
    fs::write(effects_dir.join("spiral.rhai"), EFFECT_SPIRAL)
        .map_err(|e| format!("Failed to write spiral.rhai: {}", e))?;
    fs::write(effects_dir.join("aurora.rhai"), EFFECT_AURORA)
        .map_err(|e| format!("Failed to write aurora.rhai: {}", e))?;
    fs::write(effects_dir.join("fire.rhai"), EFFECT_FIRE)
        .map_err(|e| format!("Failed to write fire.rhai: {}", e))?;
    fs::write(effects_dir.join("matrix_rain.rhai"), EFFECT_MATRIX_RAIN)
        .map_err(|e| format!("Failed to write matrix_rain.rhai: {}", e))?;
    fs::write(effects_dir.join("cyber_pulse.rhai"), EFFECT_CYBER_PULSE)
        .map_err(|e| format!("Failed to write cyber_pulse.rhai: {}", e))?;
    fs::write(effects_dir.join("music_equalizer.rhai"), EFFECT_MUSIC_EQUALIZER)
        .map_err(|e| format!("Failed to write music_equalizer.rhai: {}", e))?;

    // Create animations directory and write sample animations
    let anim_dir = install_dir.join("animations");
    fs::create_dir_all(&anim_dir)
        .map_err(|e| format!("Failed to create directory {}: {}", anim_dir.display(), e))?;
    fs::write(anim_dir.join("f87_cyber_wave.klx"), ANIM_CYBER_WAVE)
        .map_err(|e| format!("Failed to write f87_cyber_wave.klx: {}", e))?;
    fs::write(anim_dir.join("f87_gaming_zones.klx"), ANIM_GAMING_ZONES)
        .map_err(|e| format!("Failed to write f87_gaming_zones.klx: {}", e))?;
    fs::write(anim_dir.join("f87_rainbow_sweep.klx"), ANIM_RAINBOW_SWEEP)
        .map_err(|e| format!("Failed to write f87_rainbow_sweep.klx: {}", e))?;

    // Write installed marker so app detects bundled installation
    let _ = fs::write(install_dir.join("installed.txt"), "AULA F87 installed");

    // Write docs and uninstaller
    let _ = fs::write(install_dir.join("LICENSE-MIT"), LICENSE_MIT);
    let _ = fs::write(install_dir.join("LICENSE-APACHE"), LICENSE_APACHE);
    let _ = fs::write(install_dir.join("README.md"), README);
    let uninstall_bat = install_dir.join("uninstall.bat");
    let _ = fs::write(&uninstall_bat, UNINSTALL_BAT);

    // Create Desktop and Start Menu shortcuts
    if let Err(e) = create_shortcuts(&exe_path, install_dir) {
        eprintln!("Warning: shortcut creation failed: {}", e);
    }

    // Register Windows Add/Remove Programs
    register_uninstall(install_dir, &exe_path, &uninstall_bat);

    if silent {
        if launch_flag && !no_launch {
            let _ = Command::new(&exe_path).spawn();
        }
        return Ok(());
    }

    // Interactive GUI mode: ask user if they want to launch now
    let msg = format!(
        "{} has been installed successfully!\n\nLocation:\n{}\n\nShortcuts created:\n• Desktop: 'AULA F87.lnk'\n• Start Menu: 'AULA F87.lnk'\n\nWould you like to launch {} now?",
        APP_TITLE,
        install_dir.display(),
        APP_NAME
    );

    let res = show_message_box(&msg, &format!("{} Setup", APP_NAME), MB_YESNO | MB_ICONINFORMATION);
    if res == IDYES && !no_launch {
        let _ = Command::new(&exe_path).spawn();
    }

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut silent = false;
    let mut launch_flag = false;
    let mut no_launch = false;
    let mut is_uninstall = false;
    let mut custom_dir: Option<PathBuf> = None;

    let mut i = 1;
    while i < args.len() {
        let arg = &args[i];
        let lower = arg.to_lowercase();
        if lower == "--silent" || lower == "-s" || lower == "/s" || lower == "/verysilent" || lower == "/silent" {
            silent = true;
        } else if lower == "--launch" || lower == "-l" || lower == "/launch" {
            launch_flag = true;
        } else if lower == "--no-launch" || lower == "/nolaunch" || lower == "/no_launch" || lower == "--no_launch" {
            no_launch = true;
        } else if lower == "--uninstall" || lower == "/uninstall" || lower == "-u" {
            is_uninstall = true;
        } else if lower == "--dir" || lower == "-d" {
            if i + 1 < args.len() {
                i += 1;
                custom_dir = Some(PathBuf::from(&args[i]));
            }
        } else if lower.starts_with("/dir=") {
            let path_part = &arg[5..];
            let clean = path_part.trim_matches('"');
            custom_dir = Some(PathBuf::from(clean));
        } else if lower == "--help" || lower == "-h" || lower == "/?" || lower == "/help" {
            let help = format!(
                "{title} Setup v{ver}\n\n\
                Usage: AULA-F87-Setup-x64.exe [OPTIONS]\n\n\
                Options:\n  \
                  --silent, /S       Run setup silently without prompts\n  \
                  --dir <PATH>       Install to custom directory\n  \
                  --launch           Launch app immediately after install\n  \
                  --no-launch        Do not launch app after install\n  \
                  --uninstall        Uninstall previously installed application\n  \
                  --help, /?         Show this information",
                title = APP_TITLE,
                ver = VERSION
            );
            show_message_box(&help, &format!("{} Setup", APP_NAME), MB_OK | MB_ICONINFORMATION);
            return;
        }
        i += 1;
    }

    let install_dir = custom_dir.unwrap_or_else(default_install_dir);

    if is_uninstall {
        if let Err(e) = uninstall(&install_dir, silent) {
            if !silent {
                show_message_box(
                    &format!("Uninstall error: {}", e),
                    &format!("{} Setup Error", APP_NAME),
                    MB_OK | MB_ICONERROR,
                );
            }
            std::process::exit(1);
        }
        return;
    }

    if let Err(e) = install(&install_dir, silent, launch_flag, no_launch) {
        if !silent {
            show_message_box(
                &format!("Installation failed: {}", e),
                &format!("{} Setup Error", APP_NAME),
                MB_OK | MB_ICONERROR,
            );
        }
        std::process::exit(1);
    }
}
