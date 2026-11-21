// Cyberpunk Synthwave dual-color ripple pulses spreading from center.

fn meta() {
    #{
        name: "Cyber Pulse",
        description: "Dual-color neon synthwave pulses spreading from center",
        params: [
            #{ id: "speed", label: "Speed", kind: "float", min: 0.2, max: 3.0, "default": 1.0 },
            #{ id: "width", label: "Pulse Width", kind: "float", min: 0.1, max: 0.8, "default": 0.3 },
            #{ id: "color_a", label: "Neon Cyan", kind: "color", "default": "#00e5ff" },
            #{ id: "color_b", label: "Neon Pink", kind: "color", "default": "#ff007f" },
        ],
    }
}

fn render(t, keys, p) {
    let out = [];
    let pi2 = 2.0 * PI();
    for k in keys {
        let d = dist(k.nx, k.ny, 0.5, 0.5);
        let phase = (d / p.width - t * p.speed) * pi2;
        let v = 0.5 + 0.5 * sin(phase);
        let brightness = 0.0;
        if v > 0.40 {
            let f = (v - 0.40) / 0.60;
            brightness = f * f;
        }

        if brightness <= 0.0 {
            out.push([0.0, 0.0, 0.0]);
            continue;
        }

        // Dual color mix: alternate between cyan and pink along distance
        let angle = (d * 8.0 - t * p.speed * 0.5) * pi2;
        let blend = 0.5 + 0.5 * sin(angle);
        let ar = p.color_a.r * 1.0;
        let br = p.color_b.r * 1.0;
        let ag = p.color_a.g * 1.0;
        let bg = p.color_b.g * 1.0;
        let ab = p.color_a.b * 1.0;
        let bb = p.color_b.b * 1.0;
        let r = lerp(ar, br, blend) * brightness;
        let g = lerp(ag, bg, blend) * brightness;
        let b = lerp(ab, bb, blend) * brightness;

        out.push([r, g, b]);
    }
    out
}
