use crate::Model;
use nannou::color::Alpha;
use nannou::prelude::*;

const LINE_WEIGHT: f32 = 10.0;
const KEY_X: f32 = 600.0;
const KEY_Y: f32 = -200.0;
const KEY_VERT_SPACING: f32 = 30.0;

/// Render concenctric circules associated with alpha, beta, gamma..
pub fn view(app: &App, model: &Model, frame: &Frame) {
    let line_color_alpha = rgba(0.7, 0.7, 1.0, 1.0);
    let line_color_beta = rgba(0.7, 1.0, 0.7, 1.0);
    let line_color_gamma = rgba(1.0, 0.7, 0.7, 1.0);
    let line_color_delta = rgba(0.7, 1.0, 1.0, 1.0);
    let line_color_theta = rgba(1.0, 0.7, 1.0, 1.0);

    const DISTANCE: f32 = 100.0;
    const LEFT_FRONT: (f32, f32) = (-DISTANCE, -DISTANCE);
    const RIGHT_FRONT: (f32, f32) = (DISTANCE, -DISTANCE);
    const RIGHT_REAR: (f32, f32) = (DISTANCE, DISTANCE);
    const LEFT_REAR: (f32, f32) = (-DISTANCE, DISTANCE);

    let draw = app.draw();
    let background_color = BLACK;

    if (app.elapsed_frames() % 10) == 1 || model.clear_background {
        draw.background().color(background_color);
    }

    draw_key(0, "Blink", blink_color(model.blink > 0), &draw);
    draw_key(1, "Jaw Clench", blink_color(model.jaw_clench > 0), &draw);
    draw_key(2, "Alpha", line_color_alpha, &draw);
    draw_key(3, "Beta", line_color_beta, &draw);
    draw_key(4, "Gamma", line_color_gamma, &draw);
    draw_key(5, "Delta", line_color_delta, &draw);
    draw_key(6, "Theta", line_color_theta, &draw);

    draw_concentric_polygons(&app, &model, &draw, 0, LEFT_REAR);
    draw_concentric_polygons(&app, &model, &draw, 1, LEFT_FRONT);
    draw_concentric_polygons(&app, &model, &draw, 2, RIGHT_FRONT);
    draw_concentric_polygons(&app, &model, &draw, 3, RIGHT_REAR);
    // draw_polygon(
    //     line_color_alpha,
    //     model.alpha.0,
    //     &draw,
    //     app,
    //     model.scale,
    //     LEFT_REAR,
    // );
    // draw_polygon(
    //     line_color_beta,
    //     model.beta.0,
    //     &draw,
    //     app,
    //     model.scale,
    //     LEFT_REAR,
    // );
    // draw_polygon(
    //     line_color_gamma,
    //     model.gamma.0,
    //     &draw,
    //     app,
    //     model.scale,
    //     LEFT_REAR,
    // );
    // draw_polygon(
    //     line_color_delta,
    //     model.delta.0,
    //     &draw,
    //     app,
    //     model.scale,
    //     LEFT_REAR,
    // );
    // draw_polygon(
    //     line_color_theta,
    //     model.theta.0,
    //     &draw,
    //     app,
    //     model.scale,
    //     LEFT_REAR,
    // );

    // draw_polygon(
    //     line_color_alpha,
    //     model.alpha.1,
    //     &draw,
    //     app,
    //     model.scale,
    //     LEFT_FRONT,
    // );
    // draw_polygon(
    //     line_color_beta,
    //     model.beta.1,
    //     &draw,
    //     app,
    //     model.scale,
    //     LEFT_FRONT,
    // );
    // draw_polygon(
    //     line_color_gamma,
    //     model.gamma.1,
    //     &draw,
    //     app,
    //     model.scale,
    //     LEFT_FRONT,
    // );
    // draw_polygon(
    //     line_color_delta,
    //     model.delta.1,
    //     &draw,
    //     app,
    //     model.scale,
    //     LEFT_FRONT,
    // );
    // draw_polygon(
    //     line_color_theta,
    //     model.theta.1,
    //     &draw,
    //     app,
    //     model.scale,
    //     LEFT_FRONT,
    // );

    // draw_polygon(
    //     line_color_alpha,
    //     model.alpha.2,
    //     &draw,
    //     app,
    //     model.scale,
    //     RIGHT_FRONT,
    // );
    // draw_polygon(
    //     line_color_beta,
    //     model.beta.2,
    //     &draw,
    //     app,
    //     model.scale,
    //     RIGHT_FRONT,
    // );
    // draw_polygon(
    //     line_color_gamma,
    //     model.gamma.2,
    //     &draw,
    //     app,
    //     model.scale,
    //     RIGHT_FRONT,
    // );
    // draw_polygon(
    //     line_color_delta,
    //     model.delta.2,
    //     &draw,
    //     app,
    //     model.scale,
    //     RIGHT_FRONT,
    // );
    // draw_polygon(
    //     line_color_theta,
    //     model.theta.2,
    //     &draw,
    //     app,
    //     model.scale,
    //     RIGHT_FRONT,
    // );

    // draw_polygon(
    //     line_color_alpha,
    //     model.alpha.3,
    //     &draw,
    //     app,
    //     model.scale,
    //     RIGHT_REAR,
    // );
    // draw_polygon(
    //     line_color_beta,
    //     model.beta.3,
    //     &draw,
    //     app,
    //     model.scale,
    //     RIGHT_REAR,
    // );
    // draw_polygon(
    //     line_color_gamma,
    //     model.gamma.3,
    //     &draw,
    //     app,
    //     model.scale,
    //     RIGHT_REAR,
    // );
    // draw_polygon(
    //     line_color_delta,
    //     model.delta.3,
    //     &draw,
    //     app,
    //     model.scale,
    //     RIGHT_REAR,
    // );
    // draw_polygon(
    //     line_color_theta,
    //     model.theta.3,
    //     &draw,
    //     app,
    //     model.scale,
    //     RIGHT_REAR,
    // );

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
    let line_color_alpha = rgba(0.7, 0.7, 1.0, 1.0);
    let line_color_beta = rgba(0.7, 1.0, 0.7, 1.0);
    let line_color_gamma = rgba(1.0, 0.7, 0.7, 1.0);
    let line_color_delta = rgba(0.7, 1.0, 1.0, 1.0);
    let line_color_theta = rgba(1.0, 0.7, 1.0, 1.0);

    draw_polygon(
        line_color_alpha,
        model.alpha[index],
        &draw,
        app,
        model.scale,
        offset,
    );
    draw_polygon(
        line_color_beta,
        model.beta[index],
        &draw,
        app,
        model.scale,
        offset,
    );
    draw_polygon(
        line_color_gamma,
        model.gamma[index],
        &draw,
        app,
        model.scale,
        offset,
    );
    draw_polygon(
        line_color_delta,
        model.delta[index],
        &draw,
        app,
        model.scale,
        offset,
    );
    draw_polygon(
        line_color_theta,
        model.theta[index],
        &draw,
        app,
        model.scale,
        offset,
    );
}

fn blink_color(blink: bool) -> Alpha<Rgb, f32> {
    let line_color_blink = rgba(1.0, 0.2, 0.2, 1.0);
    let line_color_no_blink = rgba(0.0, 0.0, 0.0, 0.0);

    if blink {
        return line_color_blink;
    }

    line_color_no_blink
}

fn draw_key(i: i32, text: &str, line_color: Rgba, draw: &nannou::app::Draw) {
    let y = KEY_Y - KEY_VERT_SPACING * i as f32;

    draw.rect().x(KEY_X).y(y).w(50.0).h(10.0).color(line_color);

    draw.text(text).x(KEY_X).y(y - 10.0);
}

fn draw_polygon(
    line_color: Rgba,
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
