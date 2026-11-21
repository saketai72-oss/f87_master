// Flickering fireplace flames rising from the spacebar.

fn meta() {
    #{
        name: "Fire",
        description: "Warm embers and flames rising from the bottom row",
        params: [
            #{ id: "speed", label: "Speed", kind: "float", min: 0.2, max: 3.0, "default": 1.2 },
            #{ id: "heat", label: "Heat", kind: "float", min: 0.5, max: 2.0, "default": 1.1 },
            #{ id: "flicker", label: "Flicker", kind: "float", min: 0.0, max: 1.0, "default": 0.5 },
        ],
    }
}

fn render(t, keys, p) {
    let out = [];
    let pi2 = 2.0 * PI();
    for k in keys {
        // Spacebar and bottom row (ny=1.0) have lowest y_base
        let y_inv = 1.0 - k.ny;

        // Rising turbulence
        let f1 = sin((k.nx * 3.5 + t * p.speed * 0.8) * pi2);
        let f2 = cos((k.nx * 7.0 - t * p.speed * 1.5 + k.ny * 2.0) * pi2);
        let noise = (f1 + f2 * 0.5) * 0.25 * p.flicker;

        // Temperature cools towards top row
        let temp = (y_inv * 1.2 + noise) * p.heat;
        temp = clamp(temp, 0.0, 1.0);

        // Heat to color gradient:
        // Deep red (hue 0.0) -> orange -> bright yellow (hue ~0.14)
        let hue = clamp(temp * 0.14, 0.0, 0.16);
        let sat = clamp(1.0 - (temp - 0.75) * 3.0, 0.25, 1.0);
        // Clean hearth cutoff: cool air above flames stays black for 2.4G 21 FPS throughput
        let val = 0.0;
        if temp > 0.30 {
            let f = (temp - 0.30) / 0.70;
            val = f * f;
        }

        out.push(hsv(hue, sat, val));
    }
    out
}
