// Dynamic Music Equalizer & Rhythm Visualizer (Nháy theo nhạc & Phổ âm thanh)
// Simulates a 16-band audio graphic equalizer with bouncing frequency bars & beat response

fn meta() {
    #{
        name: "Music Equalizer",
        description: "16-band graphic equalizer with bouncing frequency bars & beat response",
        params: [
            #{ id: "bpm", label: "Tempo (BPM)", kind: "float", min: 60.0, max: 200.0, "default": 128.0 },
            #{ id: "sensitivity", label: "Sensitivity", kind: "float", min: 0.5, max: 2.5, "default": 1.4 },
            #{ id: "bass_boost", label: "Bass Boost", kind: "float", min: 0.5, max: 3.0, "default": 1.6 },
            #{ id: "color_low", label: "Base Color", kind: "color", "default": "#00ff88" },
            #{ id: "color_peak", label: "Peak Color", kind: "color", "default": "#ff0055" },
        ],
    }
}

fn render(t, keys, p) {
    let out = [];

    // Calculate beat clock
    let beat_rate = p.bpm / 60.0;
    let beat_t = t * beat_rate;

    // Drum rhythm components:
    // 1. Kick drum: sharp attack on every beat
    let kick_ph = frac(beat_t);
    let kick_decay = exp_decay(kick_ph * 7.0);
    let kick = kick_decay * p.bass_boost;

    // 2. Snare / Claps: on beat 2 & 4
    let snare_ph = frac(beat_t * 0.5 + 0.5);
    let snare = exp_decay(snare_ph * 8.5) * 1.2;

    // 3. Hi-hats: 16th notes fast rhythm
    let hihat_ph = frac(beat_t * 2.0);
    let hihat = exp_decay(hihat_ph * 10.0) * 0.8;

    for k in keys {
        let col = clamp(k.x, 0.0, 18.0);
        let band = col / 18.0;

        // Bass zone
        let bass_fade = 1.0 - smoothstep(0.0, 0.35, band);
        let bass_energy = kick * bass_fade;

        // Mid zone
        let mid_dist = abs(band - 0.45) * 2.5;
        let mid_atten = clamp(1.0 - mid_dist, 0.0, 1.0);
        let mid_osc = 0.5 + 0.5 * sin(beat_t * 3.1416 + col * 0.8);
        let mid_energy = snare * mid_atten + mid_osc * 0.5;

        // Treble zone
        let treb_fade = smoothstep(0.55, 1.0, band);
        let treb_osc = 0.5 + 0.5 * sin(beat_t * 6.283 + col * 1.4);
        let treb_energy = hihat * treb_fade + treb_osc * 0.4;

        let raw_energy = bass_energy + mid_energy + treb_energy;
        let total_energy = raw_energy * p.sensitivity;

        // Equalizer bar height: from 0.0 to 5.5 rows
        let bar_height = clamp(total_energy * 3.2, 0.2, 5.5);

        // Key vertical position: row 5 is bottom (0.0), row 0 is top (5.0)
        let key_height = 5.0 - k.y;

        // Check if key is within equalizer bar
        let diff = key_height - bar_height;
        let diff_abs = abs(diff);

        let brightness = 0.0;
        if diff <= 0.2 {
            let dim = (key_height / 5.5) * 0.3;
            brightness = clamp(1.0 - dim, 0.4, 1.0);
        } else if diff_abs < 0.65 {
            brightness = 1.0;
        }

        // Bass drop pulse on spacebar / bottom row keys
        if k.row == 5 && kick > 0.4 {
            brightness = clamp(brightness + kick * 0.4, 0.0, 1.0);
        }

        // Color gradient from color_low to color_peak
        let h_ratio = clamp(key_height / 5.0, 0.0, 1.0);
        let lr = p.color_low.r * 1.0;
        let pr = p.color_peak.r * 1.0;
        let lg = p.color_low.g * 1.0;
        let pg = p.color_peak.g * 1.0;
        let lb = p.color_low.b * 1.0;
        let pb = p.color_peak.b * 1.0;
        let cr = lerp(lr, pr, h_ratio) * brightness;
        let cg = lerp(lg, pg, h_ratio) * brightness;
        let cb = lerp(lb, pb, h_ratio) * brightness;

        out.push([cr, cg, cb]);
    }

    out
}

// Fast exponential decay helper using simple polynomial approximation
fn exp_decay(x) {
    if x > 3.0 {
        return 0.0;
    }
    let denom = 1.0 + x + 0.5 * x * x + 0.166 * x * x * x;
    1.0 / denom
}
