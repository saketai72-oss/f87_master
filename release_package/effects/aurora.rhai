// Northern lights / Aurora Borealis effect.
// Waves of cyan, emerald, and violet shimmering across the board.

fn meta() {
    #{
        name: "Aurora",
        description: "Shimmering curtains of arctic emerald and violet",
        params: [
            #{ id: "speed", label: "Speed", kind: "float", min: 0.1, max: 3.0, "default": 0.8 },
            #{ id: "wavelength", label: "Wavelength", kind: "float", min: 0.2, max: 2.0, "default": 0.6 },
            #{ id: "brightness", label: "Brightness", kind: "float", min: 0.2, max: 1.0, "default": 0.9 },
        ],
    }
}

fn render(t, keys, p) {
    let out = [];
    let pi2 = 2.0 * PI();
    for k in keys {
        // Multi-frequency wave pattern
        let w1 = sin((k.nx / p.wavelength - t * p.speed * 0.4) * pi2);
        let w2 = cos((k.ny * 1.5 - t * p.speed * 0.6 + k.nx * 0.5) * pi2);
        let wave = (w1 + w2) * 0.5;

        // Hue shifts between green/cyan (0.42) and purple/magenta (0.75)
        let hue = frac(0.42 + wave * 0.20);

        // High contrast arctic curtains on a dark sky for 2.4G wireless smoothness
        let v = 0.5 + 0.5 * wave;
        let edge = clamp(1.0 - abs(k.ny - 0.4) * 1.5, 0.0, 1.0);
        let raw_val = v * edge;
        let brightness = 0.0;
        if raw_val > 0.40 {
            let f = (raw_val - 0.40) / 0.60;
            brightness = f * f * p.brightness;
        }

        out.push(hsv(hue, 0.95, brightness));
    }
    out
}
