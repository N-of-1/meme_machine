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

use crate::muse_packet::*;
use nannou::prelude::*;
use nannou_osc as osc;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

mod muse_packet;
mod view_circles;

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

struct Eeg {
    a: f32,
    b: f32,
    c: f32,
    d: f32,
}

#[derive(Debug)]
pub struct Model {
    tx_eeg: Sender<Eeg>,
    rx_eeg: Receiver<Eeg>,
    receiver: ReceiverDebug,
    clicked: bool,
    clear_background: bool,
    alpha: [f32; 4],
    beta: [f32; 4],
    gamma: [f32; 4],
    delta: [f32; 4],
    theta: [f32; 4],
    blink: i32,
    batt: i32,
    accelerometer: (f32, f32, f32),
    gyro: (f32, f32, f32),
    touching_forehead: i32,
    horseshoe: (f32, f32, f32, f32),
    jaw_clench: i32,
    scale: f32,
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .with_maximized(true)
        .with_decorations(false)
        .view(view_circles::view)
        .mouse_pressed(mouse_pressed)
        .mouse_released(mouse_released)
        .key_pressed(key_pressed)
        .key_released(key_released)
        .build()
        .unwrap();

    let (tx_eeg, rx_eeg): (Sender<Eeg>, Receiver<Eeg>) = mpsc::channel();

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
        alpha: [0.0, 0.0, 0.0, 0.0], // 7.5-13Hz
        beta: [0.0, 0.0, 0.0, 0.0],  // 13-30Hz
        gamma: [0.0, 0.0, 0.0, 0.0], // 30-44Hz
        delta: [0.0, 0.0, 0.0, 0.0], // 1-4Hz
        theta: [0.0, 0.0, 0.0, 0.0], // 4-8Hz
        blink: 0,
        batt: 100,
        accelerometer: (0.0, 0.0, 0.0),
        gyro: (0.0, 0.0, 0.0),
        touching_forehead: 0,
        horseshoe: (0.0, 0.0, 0.0, 0.0),
        jaw_clench: 0,
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

    if model.blink > 0 {
        model.blink = model.blink - 1;
    }
    if model.jaw_clench > 0 {
        model.jaw_clench = model.jaw_clench - 1;
    }
}

const FOREHEAD_COUNTDOWN: i32 = 30;
const BLINK_COUNTDOWN: i32 = 30;
const CLENCH_COUNTDOWN: i32 = 30;

fn handle_packet(muse_message: &MuseMessage, model: &mut Model) {
    match muse_message.muse_message_type {
        MuseMessageType::Accelerometer { x, y, z } => {
            model.accelerometer = (x, y, z);
            //TODO Transmit accelerometer
        }
        MuseMessageType::Gyro { x, y, z } => {
            model.gyro = (x, y, z);
            //TODO Transmit gyro
        }
        MuseMessageType::TouchingForehead { value: _ } => {
            model.touching_forehead = FOREHEAD_COUNTDOWN;
            //TODO Transmit touching forehead
        }
        MuseMessageType::Horseshoe { a, b, c, d } => {
            model.horseshoe = (a, b, c, d);
            //TODO Transmit horseshoe
        }
        MuseMessageType::Eeg { a, b, c, d } => {
            model
                .tx_eeg
                .send(Eeg {
                    a: a,
                    b: b,
                    c: c,
                    d: d,
                })
                .expect("Could not send to tx_eeg");
            //TODO Transmit EEG- need to complete the receiver code
        }
        MuseMessageType::AlphaAbsolute { a, b, c, d } => {
            model.alpha = [a, b, c, d];
            //TODO Transmit alpha
        }
        MuseMessageType::BetaAbsolute { a, b, c, d } => {
            model.beta = [a, b, c, d];
            //TODO Trasmit beta
        }
        MuseMessageType::GammaAbsolute { a, b, c, d } => {
            model.gamma = [a, b, c, d];
            //TODO Transmit gamma
        }
        MuseMessageType::DeltaAbsolute { a, b, c, d } => {
            model.delta = [a, b, c, d];
            //TODO Transmit delta
        }
        MuseMessageType::ThetaAbsolute { a, b, c, d } => {
            model.theta = [a, b, c, d];
            //TODO Transmit theta
        }
        MuseMessageType::Blink { value: _ } => {
            model.blink = BLINK_COUNTDOWN;
            //TODO Transmit blink
        }
        MuseMessageType::Batt { value } => {
            model.batt = value;
            //TODO Transmit battery
        }
        MuseMessageType::JawClench { value: _ } => {
            model.jaw_clench = CLENCH_COUNTDOWN;
            //TODO Transmit jaw clench
        }
    }
}
