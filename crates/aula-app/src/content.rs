//! Installed builds keep editable content in the user's settings directory.
use std::path::{Path, PathBuf};

const EMBEDDED_EFFECTS: &[(&str, &[u8])] = &[
    (
        "breathe.rhai",
        include_bytes!("../../../effects/breathe.rhai"),
    ),
    (
        "ripple.rhai",
        include_bytes!("../../../effects/ripple.rhai"),
    ),
    (
        "spiral.rhai",
        include_bytes!("../../../effects/spiral.rhai"),
    ),
    (
        "aurora.rhai",
        include_bytes!("../../../effects/aurora.rhai"),
    ),
    ("fire.rhai", include_bytes!("../../../effects/fire.rhai")),
    (
        "matrix_rain.rhai",
        include_bytes!("../../../effects/matrix_rain.rhai"),
    ),
    (
        "cyber_pulse.rhai",
        include_bytes!("../../../effects/cyber_pulse.rhai"),
    ),
    (
        "music_equalizer.rhai",
        include_bytes!("../../../effects/music_equalizer.rhai"),
    ),
];

const EMBEDDED_ANIMATIONS: &[(&str, &[u8])] = &[
    (
        "f87_cyber_wave.klx",
        include_bytes!("../../../animations/f87_cyber_wave.klx"),
    ),
    (
        "f87_gaming_zones.klx",
        include_bytes!("../../../animations/f87_gaming_zones.klx"),
    ),
    (
        "f87_rainbow_sweep.klx",
        include_bytes!("../../../animations/f87_rainbow_sweep.klx"),
    ),
];

pub fn installed_effects_dir() -> Option<PathBuf> {
    let base = crate::settings::Settings::path()?.parent()?.join("content");
    let effects = base.join("effects");
    let anims = base.join("animations");
    let _ = std::fs::create_dir_all(&effects);
    let _ = std::fs::create_dir_all(&anims);

    // If bundled effects exist on disk alongside exe, seed from them:
    if let Ok(exe) = std::env::current_exe() {
        if let Some(parent) = exe.parent() {
            let bundled_effects = parent.join("effects");
            if bundled_effects.is_dir() {
                let _ = seed_missing(&bundled_effects, &effects);
            }
            let bundled_anims = parent.join("animations");
            if bundled_anims.is_dir() {
                let _ = seed_missing(&bundled_anims, &anims);
            }
        }
    }

    // Always ensure embedded effects are present in user content dir:
    for &(name, bytes) in EMBEDDED_EFFECTS {
        let target = effects.join(name);
        if !target.is_file() {
            let _ = std::fs::write(&target, bytes);
        }
    }
    for &(name, bytes) in EMBEDDED_ANIMATIONS {
        let target = anims.join(name);
        if !target.is_file() {
            let _ = std::fs::write(&target, bytes);
        }
    }

    Some(effects)
}

fn seed_missing(source: &Path, destination: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(destination)?;
    for entry in std::fs::read_dir(source)? {
        let entry = entry?;
        if !entry.file_type()?.is_file() {
            continue;
        }
        let target = destination.join(entry.file_name());
        // Exclusive creation preserves user edits, including during upgrades.
        match std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(target)
        {
            Ok(mut file) => {
                std::io::copy(&mut std::fs::File::open(entry.path())?, &mut file)?;
            }
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => (),
            Err(e) => return Err(e),
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn upgrades_add_missing_effects_and_preserve_user_edits() {
        let base = std::env::temp_dir().join(format!(
            "f87-content-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let source = base.join("bundled");
        let target = base.join("user");
        std::fs::create_dir_all(&source).unwrap();
        std::fs::write(source.join("breathe.rhai"), "bundled").unwrap();
        seed_missing(&source, &target).unwrap();
        std::fs::write(target.join("breathe.rhai"), "user edits").unwrap();
        std::fs::write(source.join("new.rhai"), "new effect").unwrap();
        seed_missing(&source, &target).unwrap();
        assert_eq!(
            std::fs::read_to_string(target.join("breathe.rhai")).unwrap(),
            "user edits"
        );
        assert_eq!(
            std::fs::read_to_string(target.join("new.rhai")).unwrap(),
            "new effect"
        );
        std::fs::remove_dir_all(base).unwrap();
    }
}
