#![allow(unused)]

use nannou::prelude::*;

pub struct Model {
    window: window::Id,
    pub version: usize,
    // Box state so we can migrate / recreate it
    state: Box<State>,
}

impl Model {
    pub fn new(window: WindowId) -> Self {
        Self {
            window,
            version: 0,
            state: Box::new(State::default()),
        }
    }
}

#[derive(Default, Debug)]
pub struct State {
    version: usize,
}

#[no_mangle]
pub fn event(app: &App, model: &mut Model, event: WindowEvent) {}

#[no_mangle]
pub fn update(app: &App, model: &mut Model, update: Update) {
    if model.state.version < model.version {
        println!("new version {}", model.version);
        model.state = Box::new(State {
            version: model.version,
            ..Default::default()
        });
    }
}

#[no_mangle]
pub fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);
    draw.to_frame(app, &frame).unwrap();
}
