use druid::{AppLauncher, WindowDesc};

mod data;
use data::Todos;

mod view;
use view::build_ui;

mod controllers;
mod delegate;
use delegate::Delegate;

fn main() {
    let main_window = WindowDesc::new(build_ui)
        .title("Todos")
        .resizable(true)
        .window_size((400.0, 400.0));

    let initial_state = Todos::load_from_json();

    AppLauncher::with_window(main_window)
        .delegate(Delegate {})
        .launch(initial_state)
        .expect("Failed to launch application");
}
