use crate::Model;
use nannou::prelude::*;
use std::marker::PhantomData;

const LINE_WEIGHT: f32 = 10.0;
const KEY_X: f32 = 600.0;
const KEY_Y: f32 = -100.0;
const KEY_VERT_SPACING: f32 = 30.0;

pub const _COLOR_NOF1_TURQOISE: Srgb<u8> = Srgb {
    red: 0,
    green: 200,
    blue: 200,
    standard: PhantomData,
};
pub const COLOR_NOF1_DARK_BLUE: Srgb<u8> = Srgb {
    red: 31,
    green: 18,
    blue: 71,
    standard: PhantomData,
};
pub const COLOR_NOF1_LIGHT_BLUE: Srgb<u8> = Srgb {
    red: 189,
    green: 247,
    blue: 255,
    standard: PhantomData,
};
pub const COLOR_BACKGROUND: Srgb<u8> = COLOR_NOF1_DARK_BLUE;
pub const COLOR_ALPHA: Srgb<u8> = Srgb {
    red: 178,
    green: 178,
    blue: 255,
    standard: PhantomData,
};
pub const COLOR_BETA: Srgb<u8> = Srgb {
    red: 178,
    green: 255,
    blue: 178,
    standard: PhantomData,
};
pub const COLOR_GAMMA: Srgb<u8> = Srgb {
    red: 255,
    green: 178,
    blue: 178,
    standard: PhantomData,
};
pub const COLOR_DELTA: Srgb<u8> = Srgb {
    red: 178,
    green: 255,
    blue: 255,
    standard: PhantomData,
};
pub const COLOR_THETA: Srgb<u8> = Srgb {
    red: 255,
    green: 178,
    blue: 255,
    standard: PhantomData,
};

/// Render concenctric circules associated with alpha, beta, gamma..
pub fn view(app: &App, model: &Model, frame: &Frame) {
    const DISTANCE: f32 = 100.0;
    const LEFT_FRONT: (f32, f32) = (-DISTANCE, -DISTANCE);
    const RIGHT_FRONT: (f32, f32) = (DISTANCE, -DISTANCE);
    const RIGHT_REAR: (f32, f32) = (DISTANCE, DISTANCE);
    const LEFT_REAR: (f32, f32) = (-DISTANCE, DISTANCE);

    let draw = app.draw();

    if (app.elapsed_frames() % 10) == 1 || model.clear_background {
        draw.background().color(COLOR_BACKGROUND);
    }

    draw_key(0, "Blink", blink_color(model.blink_countdown > 0), &draw);
    draw_key(
        1,
        "Jaw Clench",
        blink_color(model.jaw_clench_countdown > 0),
        &draw,
    );
    draw_key(
        2,
        "Forehead",
        blink_color(model.touching_forehead_countdown > 0),
        &draw,
    );
    draw_key(3, "Alpha", COLOR_ALPHA, &draw);
    draw_key(4, "Beta", COLOR_BETA, &draw);
    draw_key(5, "Gamma", COLOR_GAMMA, &draw);
    draw_key(6, "Delta", COLOR_DELTA, &draw);
    draw_key(7, "Theta", COLOR_THETA, &draw);

    draw_concentric_polygons(&app, &model, &draw, 0, LEFT_REAR);
    draw_concentric_polygons(&app, &model, &draw, 1, LEFT_FRONT);
    draw_concentric_polygons(&app, &model, &draw, 2, RIGHT_FRONT);
    draw_concentric_polygons(&app, &model, &draw, 3, RIGHT_REAR);

    // Write to the window frame.
    draw.to_frame(app, &frame).unwrap();
}

fn draw_concentric_polygons(
    app: &App,
    model: &Model,
    draw: &nannou::app::Draw,
    index: usize,
    offset: (f32, f32),
) {
    draw_polygon(
        COLOR_ALPHA,
        model.alpha[index],
        &draw,
        app,
        model.scale,
        offset,
    );
    draw_polygon(
        COLOR_BETA,
        model.beta[index],
        &draw,
        app,
        model.scale,
        offset,
    );
    draw_polygon(
        COLOR_GAMMA,
        model.gamma[index],
        &draw,
        app,
        model.scale,
        offset,
    );
    draw_polygon(
        COLOR_DELTA,
        model.delta[index],
        &draw,
        app,
        model.scale,
        offset,
    );
    draw_polygon(
        COLOR_THETA,
        model.theta[index],
        &draw,
        app,
        model.scale,
        offset,
    );
}

fn blink_color(blink: bool) -> Rgb<u8> {
    if blink {
        return COLOR_NOF1_LIGHT_BLUE;
    }

    COLOR_BACKGROUND
}

fn draw_key(i: i32, text: &str, line_color: Rgb<u8>, draw: &nannou::app::Draw) {
    let y = KEY_Y - KEY_VERT_SPACING * i as f32;

    draw.rect().x(KEY_X).y(y).w(50.0).h(10.0).color(line_color);

    draw.text(text).x(KEY_X).y(y - 10.0);
}

fn draw_polygon(
    line_color: Srgb<u8>,
    value: f32,
    draw: &nannou::app::Draw,
    app: &App,
    scale: f32,
    shift: (f32, f32),
) {
    let win = app.window_rect();
    let scale = win.x.end / scale;
    let circle_resolution = 256;
    let radius = value * scale;
    let angle = TAU / circle_resolution as f32;

    let mut points = Vec::new();
    for i in 0..circle_resolution {
        let x = shift.0 + (angle * i as f32).cos() * radius;
        let y = shift.1 + (angle * i as f32).sin() * radius;
        points.push(pt2(x, y));
    }

    draw.polygon()
        .stroke(line_color)
        .stroke_weight(LINE_WEIGHT)
        .no_fill()
        .points(points);
}
