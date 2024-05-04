use crate::backend::{AppRunner};
use eframe::{App, NativeOptions};
use ::egui::Response;
use egui::{EguiStuff};
use std::sync::{Arc, Mutex};

mod backend;
mod fbink;
mod egui;

pub fn start(mut app: Box<dyn App>, native_options: NativeOptions, pixel_per_point: f32) -> () {
    let mut egui_stuff: EguiStuff = EguiStuff::new(app, pixel_per_point);
    let mut runner = AppRunner::new(egui_stuff);

    loop {
        runner.next_frame();
    }
}

pub fn handle_component_update(response: Response) -> Response {
    /*
    if response.changed(){

        inkview_sys::partial_update((response.rect.min.x * response.ctx.pixels_per_point()) as i32,
                                    (response.rect.min.y * response.ctx.pixels_per_point()) as i32,
                                    (response.rect.width() * response.ctx.pixels_per_point()) as i32,
                                    (response.rect.height() * response.ctx.pixels_per_point()) as i32,

        );
    }
    */
    return response;
}
