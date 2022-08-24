#![windows_subsystem = "windows"]
use fltk::{app, button::Button, frame::Frame, prelude::*, window::Window};
use std::process::Command;
use fltk::input::Input;

fn main() {
    let app = app::App::default();
    

    let mut wind = Window::new(100, 100, 1000, 800, "MSG COPY");
    let mut but = Button::new(160, 210, 80, 40, "Click me!");
    let mut inp = Input::new(130,100,150,40,"");
    let mut fr = Frame::new(130,50,150,40,"");
    let mut butt = Button::new(200,240,100,60,"Nothing");
    but.set_callback(move |_| fr.set_label(&inp.value()));
    wind.show();
    wind.end();
    app.run().unwrap();




}



