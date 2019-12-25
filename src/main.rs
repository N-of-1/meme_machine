// P_2_0_02
//
// Generative Gestaltung – Creative Coding im Web
// ISBN: 978-3-87439-902-9, First Edition, Hermann Schmidt, Mainz, 2018
// Benedikt Groß, Hartmut Bohnacker, Julia Laub, Claudius Lazzeroni
// with contributions by Joey Lee and Niels Poldervaart
// Copyright 2018
//
// http://www.generative-gestaltung.de
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at http://www.apache.org/licenses/LICENSE-2.0
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

mod muse_packet;
mod muse_storage;
mod view_circles;

use crate::muse_packet::*;
use nannou::prelude::*;
use nannou_osc as osc;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

// Make sure this matches the `TARGET_PORT` in the `osc_sender.rs` example.
const PORT: u16 = 34254;

fn main() {
    nannou::app(model).update(update).run();
}

struct ReceiverDebug {
    receiver: osc::Receiver,
}

impl Debug for ReceiverDebug {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "<Receiver>")
    }
}

#[derive(Debug)]
pub struct Model {
    tx_eeg: Sender<MuseMessageType>,
    rx_eeg: Receiver<MuseMessageType>,
    receiver: ReceiverDebug,
    clicked: bool,
    clear_background: bool,
    accelerometer: [f32; 3],
    gyro: [f32; 3],
    alpha: [f32; 4],
    beta: [f32; 4],
    gamma: [f32; 4],
    delta: [f32; 4],
    theta: [f32; 4],
    batt: i32,
    horseshoe: [f32; 4],
    blink_countdown: i32,
    touching_forehead_countdown: i32,
    jaw_clench_countdown: i32,
    scale: f32,
}

fn model(app: &App) -> Model {
//    let monitor = winit::Window::getPrimaryMonitor();

    let _window = app
        .new_window()
//        .set_fullscreen_on_shortcut(true)
        .with_maximized(true)
        .with_decorations(false)
        .view(view_circles::view)
        .mouse_pressed(mouse_pressed)
        .mouse_released(mouse_released)
        .key_pressed(key_pressed)
        .key_released(key_released)
        .build()
        .unwrap();

    let (tx_eeg, rx_eeg): (Sender<MuseMessageType>, Receiver<MuseMessageType>) = mpsc::channel();

    // Bind an `osc::Receiver` to a port.
    let receiver = osc::receiver(PORT)
        .expect("Can not bind to port- is another copy of this app already running?");

    let receiver_debug = ReceiverDebug { receiver: receiver };

    Model {
        tx_eeg: tx_eeg,
        rx_eeg: rx_eeg,
        receiver: receiver_debug,
        clicked: false,
        clear_background: false,
        accelerometer: [0.0, 0.0, 0.0],
        gyro: [0.0, 0.0, 0.0],
        alpha: [0.0, 0.0, 0.0, 0.0], // 7.5-13Hz
        beta: [0.0, 0.0, 0.0, 0.0],  // 13-30Hz
        gamma: [0.0, 0.0, 0.0, 0.0], // 30-44Hz
        delta: [0.0, 0.0, 0.0, 0.0], // 1-4Hz
        theta: [0.0, 0.0, 0.0, 0.0], // 4-8Hz
        batt: 0,
        horseshoe: [0.0, 0.0, 0.0, 0.0],
        blink_countdown: 0,
        touching_forehead_countdown: 0,
        jaw_clench_countdown: 0,
        scale: 2.5,
    }
}

fn _cls() {
    print!("{}[2J", 27 as char);
}

fn mouse_pressed(_app: &App, model: &mut Model, _button: MouseButton) {
    model.clicked = true;
}

fn mouse_released(_app: &App, model: &mut Model, _button: MouseButton) {
    model.clicked = false;
}

fn key_pressed(_app: &App, model: &mut Model, key: Key) {
    match key {
        Key::Space => model.clear_background = !model.clear_background,
        _ => (),
    }
}

fn key_released(_app: &App, _model: &mut Model, key: Key) {
    match key {
        _ => (),
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    //     cls();
    //     println!("update: model: alpha: {:#?}", model.alpha);
    let mut received_packets = Vec::new();

    // Receive any pending osc packets.
    for (packet, addr) in model.receiver.receiver.try_iter() {
        received_packets.push((addr, packet));
    }

    for &(addr, ref packet) in received_packets.iter().rev() {
        let muse_messages = parse_muse_packet(addr, packet);

        for muse_message in muse_messages {
            handle_packet(&muse_message, model);
        }
    }

    if model.blink_countdown > 0 {
        model.blink_countdown = model.blink_countdown - 1;
    }
    if model.jaw_clench_countdown > 0 {
        model.jaw_clench_countdown = model.jaw_clench_countdown - 1;
    }
    if model.touching_forehead_countdown > 0 {
        model.touching_forehead_countdown = model.touching_forehead_countdown - 1;
    }
}

const FOREHEAD_COUNTDOWN: i32 = 30;
const BLINK_COUNTDOWN: i32 = 30;
const CLENCH_COUNTDOWN: i32 = 30;

fn handle_packet(muse_message: &MuseMessage, model: &mut Model) {
    match muse_message.muse_message_type {
        MuseMessageType::Accelerometer { x, y, z } => {
            model.accelerometer = [x, y, z];
            model
                .tx_eeg
                .send(MuseMessageType::Accelerometer { x: x, y: y, z: z })
                .expect("Could not tx Accelerometer");
        }
        MuseMessageType::Gyro { x, y, z } => {
            model.gyro = [x, y, z];
            model
                .tx_eeg
                .send(MuseMessageType::Gyro { x: x, y: y, z: z })
                .expect("Could not tx Gyro");
        }
        MuseMessageType::Horseshoe { a, b, c, d } => {
            model.horseshoe = [a, b, c, d];
            model
                .tx_eeg
                .send(MuseMessageType::Horseshoe {
                    a: a,
                    b: b,
                    c: c,
                    d: d,
                })
                .expect("Could not tx Horeshoe");
        }
        MuseMessageType::Eeg { a, b, c, d } => {
            model
                .tx_eeg
                .send(MuseMessageType::Eeg {
                    a: a,
                    b: b,
                    c: c,
                    d: d,
                })
                .expect("Could not send tx Eeg");
        }
        MuseMessageType::Alpha { a, b, c, d } => {
            model.alpha = [a, b, c, d];
            model
                .tx_eeg
                .send(MuseMessageType::Alpha {
                    a: a,
                    b: b,
                    c: c,
                    d: d,
                })
                .expect("Could not send tx Alpha");
        }
        MuseMessageType::Beta { a, b, c, d } => {
            model.beta = [a, b, c, d];
            model
                .tx_eeg
                .send(MuseMessageType::Beta {
                    a: a,
                    b: b,
                    c: c,
                    d: d,
                })
                .expect("Could not send tx Beta");
        }
        MuseMessageType::Gamma { a, b, c, d } => {
            model.gamma = [a, b, c, d];
            model
                .tx_eeg
                .send(MuseMessageType::Gamma {
                    a: a,
                    b: b,
                    c: c,
                    d: d,
                })
                .expect("Could not send tx Gamma");
        }
        MuseMessageType::Delta { a, b, c, d } => {
            model.delta = [a, b, c, d];
            model
                .tx_eeg
                .send(MuseMessageType::Delta {
                    a: a,
                    b: b,
                    c: c,
                    d: d,
                })
                .expect("Could not send tx Delta");
        }
        MuseMessageType::Theta { a, b, c, d } => {
            model.theta = [a, b, c, d];
            model
                .tx_eeg
                .send(MuseMessageType::Theta {
                    a: a,
                    b: b,
                    c: c,
                    d: d,
                })
                .expect("Could not send tx Theta");
        }
        MuseMessageType::Batt { batt: batt } => {
            model.batt = batt;
            model
                .tx_eeg
                .send(MuseMessageType::Batt { batt: batt })
                .expect("Could not tx Batt");
        }
        MuseMessageType::TouchingForehead { touch: touch } => {
            if !touch {
                model.touching_forehead_countdown = FOREHEAD_COUNTDOWN;
            }
            model
                .tx_eeg
                .send(MuseMessageType::TouchingForehead { touch: touch })
                .expect("Could not tx TouchingForehead");
        }
        MuseMessageType::Blink { blink: blink } => {
            if blink {
                model.blink_countdown = BLINK_COUNTDOWN;
            }
            model
                .tx_eeg
                .send(MuseMessageType::Blink { blink: blink })
                .expect("Could not tx Blink");
        }
        MuseMessageType::JawClench { clench: clench } => {
            if clench {
                model.jaw_clench_countdown = CLENCH_COUNTDOWN;
            }
            model
                .tx_eeg
                .send(MuseMessageType::JawClench { clench: clench })
                .expect("Could not tx Clench");
        }
    }
}
