//! Before this module the status and board colours were inline `Color32`
//! literals repeated across `ui.rs` and `editor.rs`, which drifted apart and
//! made a coherent look impossible. Everything visual now names a token here,
//! so the whole app can be reskinned in one place.

use eframe::egui::{self, Color32};
use std::sync::RwLock;

/// Use `RwLock` for blocking access to current palette
static CURRENT: RwLock<&'static Palette> = RwLock::new(&DARK);

/// Named colour tokens. Dark/Light-first, matching the app's existing look, but
/// centralised so a other [`Palette`]-s could define an another theme later.
pub struct Palette {
    /// Is this theme dark.
    pub dark_mode: bool,
    /// Window background, behind every panel. Also a `TextEdit` background.
    pub bg: Color32,
    /// Panel and card fill.
    pub surface: Color32,
    /// Slightly changed fill: buttons, selected rows, inset wells.
    pub surface_alt: Color32,
    /// Buttons background when hover
    pub hovered_bg: Color32,
    pub text: Color32,
    pub text_muted: Color32,
    /// Primary cyber accent (cyan #00e5ff).
    pub accent: Color32,
    /// Secondary cyber accent (neon violet #7c4dff).
    pub accent_secondary: Color32,
    pub accent_fill: Color32,
    pub on_accent: Color32,
    /// Connected, healthy.
    pub success: Color32,
    /// Waiting, unsaved, soft warning.
    pub warning: Color32,
    /// Disconnected, error.
    pub danger: Color32,
    /// An unlit key on the board preview — visible so it still reads as a key.
    pub key_unlit: Color32,
    /// Hairline around each key.
    pub key_stroke: Color32,
    /// The board's own backdrop.
    pub board_bg: Color32,
}

/// The default dark palette (sleek cyber-dark obsidian).
pub const DARK: Palette = Palette {
    dark_mode: true,
    bg: Color32::from_rgb(13, 15, 18),      // deep obsidian #0d0f12
    surface: Color32::from_rgb(21, 24, 30), // surface #15181e
    surface_alt: Color32::from_rgb(28, 33, 41), // surface_alt #1c2129
    hovered_bg: Color32::from_rgb(38, 45, 56),
    text: Color32::from_rgb(240, 243, 248),
    text_muted: Color32::from_rgb(148, 163, 184),
    accent: Color32::from_rgb(0, 229, 255), // cyan #00e5ff
    accent_secondary: Color32::from_rgb(124, 77, 255), // neon violet #7c4dff
    accent_fill: Color32::from_rgb(52, 28, 108), // deep cyber violet fill
    on_accent: Color32::WHITE,
    success: Color32::from_rgb(0, 230, 153), // cyber mint
    warning: Color32::from_rgb(255, 179, 0), // cyber amber
    danger: Color32::from_rgb(255, 61, 87),  // cyber neon red
    key_unlit: Color32::from_rgb(25, 29, 36),
    key_stroke: Color32::from_rgb(42, 50, 61), // refined key stroke
    board_bg: Color32::from_rgb(16, 18, 22),   // refined board bg
};

/// The light palette.
pub const LIGHT: Palette = Palette {
    dark_mode: false,
    bg: Color32::from_rgb(241, 243, 247),
    surface: Color32::from_rgb(252, 253, 255),
    surface_alt: Color32::from_rgb(230, 234, 241),
    hovered_bg: Color32::from_rgb(216, 225, 238),
    text: Color32::from_rgb(30, 37, 49),
    text_muted: Color32::from_rgb(85, 99, 118),
    accent: Color32::from_rgb(0, 112, 144),
    accent_secondary: Color32::from_rgb(106, 27, 154),
    accent_fill: Color32::from_rgb(0, 112, 144),
    on_accent: Color32::WHITE,
    success: Color32::from_rgb(24, 116, 63),
    warning: Color32::from_rgb(137, 88, 9),
    danger: Color32::from_rgb(176, 43, 49),
    key_unlit: Color32::from_rgb(241, 243, 245),
    key_stroke: Color32::from_rgb(217, 222, 229),
    board_bg: Color32::from_rgb(250, 251, 252),
};

/// Return the current selected palette
pub fn current() -> &'static Palette {
    *CURRENT.read().unwrap()
}

/// Switch the active palette.
pub fn set_current(theme: crate::settings::ColorTheme) {
    use crate::settings::ColorTheme;

    let pal = match theme {
        ColorTheme::Dark => &DARK,
        ColorTheme::Light => &LIGHT,
    };
    *CURRENT.write().unwrap() = pal;
}

impl Palette {
    /// Apply the palette to egui's global style: panel fills, selection colour,
    /// rounded widgets, and a touch more breathing room than the default.
    pub fn apply(&self, ctx: &egui::Context) {
        let mut style = (*ctx.style()).clone();
        // Reset widget strokes and fills too: changing dark_mode alone leaves
        // the previous theme's text, popup and checkbox colours in place.
        style.visuals = if self.dark_mode {
            egui::Visuals::dark()
        } else {
            egui::Visuals::light()
        };
        let v = &mut style.visuals;

        v.dark_mode = self.dark_mode;
        v.override_text_color = None;
        v.panel_fill = self.surface;
        v.window_fill = self.surface;
        v.extreme_bg_color = self.bg;
        v.faint_bg_color = self.surface_alt;
        v.selection.bg_fill = self.accent_fill;
        v.selection.stroke = egui::Stroke::new(1.0_f32, self.on_accent);
        v.hyperlink_color = self.accent;

        let rounding = egui::Rounding::same(6.0);
        for w in [
            &mut v.widgets.noninteractive,
            &mut v.widgets.inactive,
            &mut v.widgets.hovered,
            &mut v.widgets.active,
            &mut v.widgets.open,
        ] {
            w.rounding = rounding;
            w.fg_stroke.color = self.text;
        }

        // Enable left side of slider fill (which is disabled in light mode by default)
        v.slider_trailing_fill = true;

        // Buttons and combo-box
        v.widgets.inactive.weak_bg_fill = self.surface_alt;
        v.widgets.hovered.weak_bg_fill = self.hovered_bg;
        v.widgets.active.weak_bg_fill = self.hovered_bg;
        v.widgets.noninteractive.weak_bg_fill = self.hovered_bg; // disabled buttons
        v.widgets.open.weak_bg_fill = self.surface_alt;
        v.widgets.hovered.bg_stroke = egui::Stroke::new(1.0_f32, self.accent);
        v.widgets.active.bg_stroke = egui::Stroke::new(1.5_f32, self.accent);

        // Slider dot color when interacting
        v.widgets.active.bg_fill = self.accent;

        // Separators color
        v.widgets.noninteractive.bg_stroke = egui::Stroke::new(1.0_f32, self.key_stroke);

        style.spacing.item_spacing = egui::vec2(10.0, 10.0);
        style.spacing.button_padding = egui::vec2(12.0, 7.0);
        style.spacing.interact_size = egui::vec2(32.0, 32.0);
        style.spacing.slider_width = 180.0;
        style.spacing.combo_width = 160.0;
        style.spacing.text_edit_width = 200.0;
        style
            .text_styles
            .insert(egui::TextStyle::Heading, egui::FontId::proportional(25.0));
        style
            .text_styles
            .insert(egui::TextStyle::Body, egui::FontId::proportional(14.0));
        style
            .text_styles
            .insert(egui::TextStyle::Button, egui::FontId::proportional(14.0));
        style
            .text_styles
            .insert(egui::TextStyle::Small, egui::FontId::proportional(12.0));
        style.animation_time = 0.10;

        ctx.set_style(style);
    }
}

/// Use an installed platform face when available; keep egui's Unicode fallbacks.
pub fn install_fonts(ctx: &egui::Context) {
    let candidates = if cfg!(target_os = "windows") {
        vec!["C:/Windows/Fonts/segoeui.ttf"]
    } else if cfg!(target_os = "macos") {
        vec!["/System/Library/Fonts/SFNS.ttf"]
    } else {
        vec!["/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf"]
    };
    for path in candidates {
        if let Ok(bytes) = std::fs::read(path) {
            let mut fonts = egui::FontDefinitions::default();
            fonts
                .font_data
                .insert("platform-ui".into(), egui::FontData::from_owned(bytes));
            fonts
                .families
                .entry(egui::FontFamily::Proportional)
                .or_default()
                .insert(0, "platform-ui".into());
            ctx.set_fonts(fonts);
            break;
        }
    }
}

pub fn primary_button(label: &str) -> egui::Button<'_> {
    let pal = current();
    egui::Button::new(egui::RichText::new(label).color(pal.on_accent).strong())
        .fill(pal.accent_fill)
        .stroke(egui::Stroke::new(1.0_f32, pal.accent))
        .rounding(6.0)
        .wrap_mode(egui::TextWrapMode::Extend)
}

pub fn section(ui: &mut egui::Ui, title: &str, subtitle: &str) {
    ui.label(egui::RichText::new(title).size(17.0).strong());
    if !subtitle.is_empty() {
        muted(ui, subtitle);
    }
    ui.add_space(6.0);
}

pub fn content_frame() -> egui::Frame {
    egui::Frame::none()
        .fill(current().surface)
        .rounding(10.0)
        .inner_margin(18.0)
}

pub fn surface<R>(
    ui: &mut egui::Ui,
    add_contents: impl FnOnce(&mut egui::Ui) -> R,
) -> egui::InnerResponse<R> {
    let width = (ui.available_width() - 36.0).max(0.0);
    content_frame().show(ui, |ui| {
        ui.set_min_width(width);
        add_contents(ui)
    })
}

pub fn page_heading(ui: &mut egui::Ui, title: &str, subtitle: &str) {
    ui.heading(title);
    muted(ui, subtitle);
    ui.add_space(12.0);
}

pub fn muted(ui: &mut egui::Ui, text: impl Into<String>) -> egui::Response {
    ui.label(egui::RichText::new(text).color(current().text_muted))
}

pub fn color_edit(ui: &mut egui::Ui, label: &str, rgb: &mut [u8; 3]) -> egui::Response {
    let response = ui.color_edit_button_srgb(rgb);
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Button, ui.is_enabled(), label)
    });
    response.on_hover_text(label)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn switching_themes_resets_widget_and_popup_visuals() {
        let ctx = egui::Context::default();
        for palette in [&LIGHT, &DARK, &LIGHT] {
            palette.apply(&ctx);
            let style = ctx.style();
            let expected = if palette.dark_mode {
                egui::Visuals::dark()
            } else {
                egui::Visuals::light()
            };
            assert_eq!(style.visuals.dark_mode, palette.dark_mode);
            assert_eq!(style.visuals.override_text_color, None);
            assert_eq!(style.visuals.window_stroke, expected.window_stroke);
            assert_eq!(style.visuals.widgets.inactive.fg_stroke.color, palette.text);
            assert_eq!(
                style.visuals.widgets.hovered.bg_stroke,
                egui::Stroke::new(1.0_f32, palette.accent)
            );
        }
    }

    #[test]
    fn text_and_primary_actions_meet_aa_contrast_in_both_themes() {
        let luminance = |color: Color32| {
            let linear = |channel: u8| {
                let channel = f32::from(channel) / 255.0;
                if channel <= 0.04045 {
                    channel / 12.92
                } else {
                    ((channel + 0.055) / 1.055).powf(2.4)
                }
            };
            0.2126 * linear(color.r()) + 0.7152 * linear(color.g()) + 0.0722 * linear(color.b())
        };
        let contrast = |a: Color32, b: Color32| {
            let (a, b) = (luminance(a), luminance(b));
            (a.max(b) + 0.05) / (a.min(b) + 0.05)
        };
        for palette in [&DARK, &LIGHT] {
            for surface in [palette.bg, palette.surface, palette.surface_alt] {
                for text in [palette.text, palette.text_muted, palette.accent] {
                    assert!(
                        contrast(text, surface) >= 4.5,
                        "Text contrast below AA: {:?} on {:?}",
                        text,
                        surface
                    );
                }
            }
            assert!(contrast(palette.on_accent, palette.accent_fill) >= 4.5);
        }
    }
}
