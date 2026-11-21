//! AULA F87 LED index map and physical geometry.
//!
//! ANSI 87-Key Tenkeyless (TKL) layout.
//! Hardware matrix is column-major with stride 6 (rows 0..5, columns 0..16).
//! LED indices fit within the 126 slots of the streaming payload (max index 101).

use crate::device::KeyPos;

pub const KEY_COUNT: usize = 87;
pub const MAX_LED_INDEX: usize = 101;

/// LED index -> key name, straight from the firmware's matrix table.
#[rustfmt::skip]
pub const LED_ORDER: [Option<&str>; 102] = [
    // Column 0
    Some("Esc"), Some("`"), Some("Tab"), Some("Caps"), Some("LShift"), Some("LCtrl"),
    // Column 1
    None, Some("1"), Some("Q"), Some("A"), Some("Z"), Some("LWin"),
    // Column 2
    Some("F1"), Some("2"), Some("W"), Some("S"), Some("X"), Some("LAlt"),
    // Column 3
    Some("F2"), Some("3"), Some("E"), Some("D"), Some("C"), None,
    // Column 4
    Some("F3"), Some("4"), Some("R"), Some("F"), Some("V"), None,
    // Column 5
    Some("F4"), Some("5"), Some("T"), Some("G"), Some("B"), Some("Space"),
    // Column 6
    Some("F5"), Some("6"), Some("Y"), Some("H"), Some("N"), None,
    // Column 7
    Some("F6"), Some("7"), Some("U"), Some("J"), Some("M"), None,
    // Column 8
    Some("F7"), Some("8"), Some("I"), Some("K"), Some(","), Some("RAlt"),
    // Column 9
    Some("F8"), Some("9"), Some("O"), Some("L"), Some("."), Some("Fn"),
    // Column 10
    Some("F9"), Some("0"), Some("P"), Some(";"), Some("/"), Some("App"),
    // Column 11
    Some("F10"), Some("-"), Some("["), Some("'"), None, None,
    // Column 12
    Some("F11"), Some("="), Some("]"), None, None, None,
    // Column 13
    Some("F12"), Some("Backspace"), Some("\\"), Some("Enter"), Some("RShift"), Some("RCtrl"),
    // Column 14 (Nav Left / Arrow Left)
    Some("PrintScreen"), Some("Insert"), Some("Del"), None, None, Some("Left"),
    // Column 15 (Nav Mid / Arrow Up / Down)
    Some("ScrollLock"), Some("Home"), Some("End"), None, Some("Up"), Some("Down"),
    // Column 16 (Nav Right / Arrow Right)
    Some("Pause"), Some("PgUp"), Some("PgDn"), None, None, Some("Right"),
];

/// Complete 87-key ANSI TKL matrix layout: (name, led, row, x, w).
#[rustfmt::skip]
pub const KEY_MATRIX: [(&str, usize, u8, f32, f32); KEY_COUNT] = [
    // --- Row 0: Function Row & TKL Top (16 keys) ---
    ("Esc", 0, 0, 0.5, 1.0),
    ("F1", 12, 0, 2.5, 1.0),
    ("F2", 18, 0, 3.5, 1.0),
    ("F3", 24, 0, 4.5, 1.0),
    ("F4", 30, 0, 5.5, 1.0),
    ("F5", 36, 0, 7.0, 1.0),
    ("F6", 42, 0, 8.0, 1.0),
    ("F7", 48, 0, 9.0, 1.0),
    ("F8", 54, 0, 10.0, 1.0),
    ("F9", 60, 0, 11.5, 1.0),
    ("F10", 66, 0, 12.5, 1.0),
    ("F11", 72, 0, 13.5, 1.0),
    ("F12", 78, 0, 14.5, 1.0),
    ("PrintScreen", 84, 0, 16.0, 1.0),
    ("ScrollLock", 90, 0, 17.0, 1.0),
    ("Pause", 96, 0, 18.0, 1.0),

    // --- Row 1: Number Row & TKL Nav Top (17 keys) ---
    ("`", 1, 1, 0.5, 1.0),
    ("1", 7, 1, 1.5, 1.0),
    ("2", 13, 1, 2.5, 1.0),
    ("3", 19, 1, 3.5, 1.0),
    ("4", 25, 1, 4.5, 1.0),
    ("5", 31, 1, 5.5, 1.0),
    ("6", 37, 1, 6.5, 1.0),
    ("7", 43, 1, 7.5, 1.0),
    ("8", 49, 1, 8.5, 1.0),
    ("9", 55, 1, 9.5, 1.0),
    ("0", 61, 1, 10.5, 1.0),
    ("-", 67, 1, 11.5, 1.0),
    ("=", 73, 1, 12.5, 1.0),
    ("Backspace", 79, 1, 14.5, 2.0),
    ("Insert", 85, 1, 16.0, 1.0),
    ("Home", 91, 1, 17.0, 1.0),
    ("PgUp", 97, 1, 18.0, 1.0),

    // --- Row 2: QWERTY Row & TKL Nav Mid (17 keys) ---
    ("Tab", 2, 2, 0.75, 1.5),
    ("Q", 8, 2, 2.0, 1.0),
    ("W", 14, 2, 3.0, 1.0),
    ("E", 20, 2, 4.0, 1.0),
    ("R", 26, 2, 5.0, 1.0),
    ("T", 32, 2, 6.0, 1.0),
    ("Y", 38, 2, 7.0, 1.0),
    ("U", 44, 2, 8.0, 1.0),
    ("I", 50, 2, 9.0, 1.0),
    ("O", 56, 2, 10.0, 1.0),
    ("P", 62, 2, 11.0, 1.0),
    ("[", 68, 2, 12.0, 1.0),
    ("]", 74, 2, 13.0, 1.0),
    ("\\", 80, 2, 14.25, 1.5),
    ("Del", 86, 2, 16.0, 1.0),
    ("End", 92, 2, 17.0, 1.0),
    ("PgDn", 98, 2, 18.0, 1.0),

    // --- Row 3: Home Row (13 keys) ---
    ("Caps", 3, 3, 0.875, 1.75),
    ("A", 9, 3, 2.25, 1.0),
    ("S", 15, 3, 3.25, 1.0),
    ("D", 21, 3, 4.25, 1.0),
    ("F", 27, 3, 5.25, 1.0),
    ("G", 33, 3, 6.25, 1.0),
    ("H", 39, 3, 7.25, 1.0),
    ("J", 45, 3, 8.25, 1.0),
    ("K", 51, 3, 9.25, 1.0),
    ("L", 57, 3, 10.25, 1.0),
    (";", 63, 3, 11.25, 1.0),
    ("'", 69, 3, 12.25, 1.0),
    ("Enter", 81, 3, 14.375, 2.25),

    // --- Row 4: Shift Row & Up Arrow (13 keys) ---
    ("LShift", 4, 4, 1.125, 2.25),
    ("Z", 10, 4, 2.75, 1.0),
    ("X", 16, 4, 3.75, 1.0),
    ("C", 22, 4, 4.75, 1.0),
    ("V", 28, 4, 5.75, 1.0),
    ("B", 34, 4, 6.75, 1.0),
    ("N", 40, 4, 7.75, 1.0),
    ("M", 46, 4, 8.75, 1.0),
    (",", 52, 4, 9.75, 1.0),
    (".", 58, 4, 10.75, 1.0),
    ("/", 64, 4, 11.75, 1.0),
    ("RShift", 82, 4, 13.625, 2.75),
    ("Up", 94, 4, 17.0, 1.0),

    // --- Row 5: Bottom Row & Left/Down/Right Arrows (11 keys) ---
    ("LCtrl", 5, 5, 0.625, 1.25),
    ("LWin", 11, 5, 1.875, 1.25),
    ("LAlt", 17, 5, 3.125, 1.25),
    ("Space", 35, 5, 7.0, 6.25),
    ("RAlt", 53, 5, 10.875, 1.25),
    ("Fn", 59, 5, 12.125, 1.25),
    ("App", 65, 5, 13.375, 1.25),
    ("RCtrl", 83, 5, 14.625, 1.25),
    ("Left", 89, 5, 16.0, 1.0),
    ("Down", 95, 5, 17.0, 1.0),
    ("Right", 101, 5, 18.0, 1.0),
];

/// Build the layout: all 87 keys with physical position and LED index.
pub fn layout() -> Vec<KeyPos> {
    KEY_MATRIX
        .iter()
        .map(|&(name, led, row, x, w)| KeyPos {
            name: name.to_string(),
            row,
            x,
            w,
            led,
        })
        .collect()
}

/// Get key LED index by name (case-insensitive, aliases supported).
pub fn led_for(name: &str) -> Option<usize> {
    let lower = name.to_lowercase();
    KEY_MATRIX.iter().find_map(|&(k_name, led, _, _, _)| {
        if k_name.to_lowercase() == lower {
            Some(led)
        } else {
            match (k_name, lower.as_str()) {
                ("PrintScreen", "prtsc" | "prtscn" | "print") => Some(led),
                ("ScrollLock", "scrlk" | "scroll") => Some(led),
                ("Pause", "pausebreak" | "break") => Some(led),
                ("Insert", "ins") => Some(led),
                ("Del", "delete") => Some(led),
                ("PgUp", "pageup") => Some(led),
                ("PgDn", "pagedown") => Some(led),
                ("`", "tilde" | "backtick" | "~") => Some(led),
                ("Space", "spacebar") => Some(led),
                ("LWin", "win" | "super" | "gui") => Some(led),
                ("App", "menu") => Some(led),
                _ => None,
            }
        }
    })
}

/// Rightmost key centre, in 1u units. Effects use it to normalise across the board.
pub fn max_x(keys: &[KeyPos]) -> f32 {
    keys.iter().map(|k| k.x).fold(0.0, f32::max)
}

/// Bottom row index.
pub fn max_row(keys: &[KeyPos]) -> u8 {
    keys.iter().map(|k| k.row).max().unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::f75::protocol::STREAM_SLOTS;

    #[test]
    fn esc_is_led_zero_and_backtick_is_one() {
        assert_eq!(led_for("Esc"), Some(0));
        assert_eq!(led_for("`"), Some(1));
    }

    #[test]
    fn column_major_stride_of_six() {
        assert_eq!(led_for("F1"), Some(12));
        assert_eq!(led_for("F2"), Some(18));
        assert_eq!(led_for("F3"), Some(24));
    }

    #[test]
    fn minus_has_an_led_but_knob_does_not() {
        assert_eq!(led_for("-"), Some(67));
        assert_eq!(led_for("Knob"), None);
    }

    #[test]
    fn layout_is_populated_and_unique() {
        let keys = layout();
        assert_eq!(keys.len(), KEY_COUNT, "AULA F87 must have exactly 87 keys");
        let mut leds: Vec<_> = keys.iter().map(|k| k.led).collect();
        leds.sort_unstable();
        let before = leds.len();
        leds.dedup();
        assert_eq!(before, leds.len(), "LED indices must be unique");

        for k in &keys {
            assert!(
                k.led < STREAM_SLOTS,
                "Key '{}' LED index {} exceeds STREAM_SLOTS ({STREAM_SLOTS})",
                k.name,
                k.led
            );
            assert!(k.led <= MAX_LED_INDEX);
            assert!(k.row <= 5);
            assert!(k.x >= 0.0);
            assert!(k.w > 0.0);
        }
    }

    #[test]
    fn geometry_is_sane() {
        let keys = layout();
        assert_eq!(max_row(&keys), 5, "F87 has six rows (0..5)");
        let mx = max_x(&keys);
        assert!((17.0..19.0).contains(&mx), "board is ~18u wide, got {mx}");
    }

    #[test]
    fn spacebar_is_wide_enough_to_exclude() {
        let keys = layout();
        let space = keys.iter().find(|k| k.name == "Space").unwrap();
        assert!(space.w >= 3.0, "effects rely on w to skip oversized keys");
    }

    #[test]
    fn tkl_nav_and_arrow_clusters() {
        let keys = layout();
        let backspace = keys.iter().find(|k| k.name == "Backspace").unwrap();
        let insert = keys.iter().find(|k| k.name == "Insert").unwrap();
        let del = keys.iter().find(|k| k.name == "Del").unwrap();
        let home = keys.iter().find(|k| k.name == "Home").unwrap();
        let end = keys.iter().find(|k| k.name == "End").unwrap();
        let pgup = keys.iter().find(|k| k.name == "PgUp").unwrap();
        let pgdn = keys.iter().find(|k| k.name == "PgDn").unwrap();

        // Row 1 nav keys sit after Backspace
        assert_eq!(insert.row, backspace.row);
        assert!(insert.x > backspace.x);
        assert_eq!(home.row, 1);
        assert_eq!(pgup.row, 1);

        // Row 2 nav keys sit below row 1
        assert_eq!(del.row, 2);
        assert_eq!(end.row, 2);
        assert_eq!(pgdn.row, 2);
        assert_eq!(del.x, insert.x);
        assert_eq!(end.x, home.x);
        assert_eq!(pgdn.x, pgup.x);

        // Arrows inverted-T
        let up = keys.iter().find(|k| k.name == "Up").unwrap();
        let down = keys.iter().find(|k| k.name == "Down").unwrap();
        let left = keys.iter().find(|k| k.name == "Left").unwrap();
        let right = keys.iter().find(|k| k.name == "Right").unwrap();

        assert_eq!(up.row, 4);
        assert_eq!(down.row, 5);
        assert_eq!(left.row, 5);
        assert_eq!(right.row, 5);
        assert_eq!(up.x, down.x);
        assert!(left.x < down.x);
        assert!(right.x > down.x);
    }

    #[test]
    fn led_order_matches_key_matrix() {
        for &(name, led, _, _, _) in KEY_MATRIX.iter() {
            assert_eq!(LED_ORDER[led], Some(name));
            assert_eq!(led_for(name), Some(led));
        }
    }
}
