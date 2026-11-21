// Matrix phosphor-green code streams cascading down keyboard columns.

fn meta() {
    #{
        name: "Matrix Rain",
        description: "Phosphor-green digital code stream cascading down columns",
        params: [
            #{ id: "speed", label: "Speed", kind: "float", min: 0.5, max: 4.0, "default": 1.5 },
            #{ id: "trail", label: "Trail Length", kind: "float", min: 0.1, max: 0.8, "default": 0.4 },
            #{ id: "hue", label: "Hue", kind: "float", min: 0.0, max: 1.0, "default": 0.33 },
        ],
    }
}

fn render(t, keys, p) {
    let out = [];
    for k in keys {
        // Divide into column buckets
        let col = (k.nx * 18.0).floor();
        let phase = frac(col * 0.382);
        let col_speed = 0.7 + frac(col * 0.618) * 0.6;

        // Position of drop head (0.0 top -> 1.0 bottom)
        let drop_pos = frac(t * p.speed * 0.5 * col_speed + phase);

        let dist = drop_pos - k.ny;
        if dist < 0.0 {
            dist = dist + 1.0;
        }

        let v = 0.0;
        let s = 1.0;
        if dist < 0.06 {
            // Bright white-green leading drop head
            v = 1.0;
            s = dist / 0.06 * 0.7 + 0.3;
        } else if dist < p.trail {
            // Fading phosphor green trail
            let f = 1.0 - (dist - 0.06) / (p.trail - 0.06);
            v = f * f * 0.85;
            s = 1.0;
        }

        out.push(hsv(p.hue, s, v));
    }
    out
}
