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

        // Intensity fades out towards bottom/top edges smoothly
        let v = (0.5 + 0.5 * wave) * p.brightness;
        v = v * (0.3 + 0.7 * clamp(1.0 - abs(k.ny - 0.4) * 1.4, 0.0, 1.0));

        out.push(hsv(hue, 0.95, v));
    }
    out
}
