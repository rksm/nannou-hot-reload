use hot_lib::*;

#[hot_lib_reloader::hot_module(dylib = "lib", file_watch_debounce = 50)]
mod hot_lib {
    pub use lib::*;
    pub use nannou::prelude::*;

    hot_functions_from_file!("lib/src/lib.rs");

    #[lib_updated]
    pub fn was_updated() -> bool {}
}

fn model(app: &nannou::App) -> Model {
    Model::for_window(app.new_window().view(view).event(event).build().unwrap())
}

pub fn update(app: &App, model: &mut Model, update: Update) {
    model.was_updated = hot_lib::was_updated();
    hot_lib::update(app, model, update)
}

fn main() {
    nannou::app(model).update(update).run();
}
