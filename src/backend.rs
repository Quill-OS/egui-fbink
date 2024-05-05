use std::collections::HashMap;
use std::hash::{BuildHasherDefault, Hash};
use ::std::os::raw::c_int;
use eframe::{App, IntegrationInfo};
use egui::epaint::{text, ClippedShape};
use egui::{output, Context, Event, FullOutput, RawInput, Rect, Shape, ViewportId, ViewportInfo};
use egui::{PointerButton, Pos2, TouchDeviceId, TouchId, TouchPhase, Vec2};
use fbink_sys::*;
use log::{debug, error};
use raw_window_handle::HandleError;
use std::fs;
use std::ptr::null;
use std::sync::atomic::Ordering::SeqCst;
use std::sync::Arc;
use std::{ffi::CString, process::exit};

use crate::egui::EguiStuff;
use crate::fbink::FBInkBackend;

pub struct AppRunner {
    fb: FBInkBackend,
    egui: EguiStuff,
}

impl AppRunner {
    pub(crate) fn new(egui: EguiStuff) -> Self {
        let mut runner = Self {
            fb: FBInkBackend::new(),
            egui,
        };
        /*
        // gone?
        runner
            .app
            .setup(&runner.fbink_backend.egui_ctx, &frame_data, None);
        */
        runner
    }

    pub fn draw_shapes(&mut self, clipped_shapes: Vec<ClippedShape>) {
        let ppp = self.egui.pixel_per_point;
        debug!("draw shapes pixel per point: {}", ppp);
        for shape in clipped_shapes {
            if shape.clip_rect.is_negative() {
                error!("clip rect is negative");
                continue
            } else {
                // debug!("shape.0: {:?}", shape.0);
            }
            match shape.shape {
                Shape::Noop => {}
                Shape::Vec(_) => {}
                Shape::Circle(circle_shape) => {

                }
                Shape::LineSegment { .. } => {}
                Shape::Path { .. } => {}
                Shape::Rect(rect_shape) => {
                    /*
                    inkview_sys::fill_area(
                        (rect.min.x * ppp) as i32,
                        (rect.min.y * ppp) as i32,
                        (rect.width() * ppp) as i32,
                        (rect.height() * ppp) as i32,
                        inkview_sys::Color::rgb(fill.r(), fill.g(), fill.b()),
                    );
                    */

                    /*inkview_sys::draw_rect_round(rect.min.x as i32,
                                                 rect.min.y as i32,
                                                 rect.width() as i32,
                                                 rect.height() as i32,
                        inkview_sys::Color::rgb(fill.r(), fill.g(), fill.b()), corner_radius as i32
                    );*/

                    debug!("Printing out rectangle at {:?}", rect_shape);
                    let fbink_rect: FBInkRect = FBInkRect {
                        left: rect_shape.rect.left() as u16,
                        top: rect_shape.rect.top() as u16,
                        width: rect_shape.rect.width() as u16,
                        height: rect_shape.rect.height() as u16,
                    };
                    let r: u8 = rect_shape.fill.r();
                    let g: u8 = rect_shape.fill.g();
                    let b: u8 = rect_shape.fill.b();
                    let a: u8 = rect_shape.fill.a();

                    unsafe {
                        if fbink_fill_rect_rgba(self.fb.fbfd, &self.fb.fbink_cfg, &fbink_rect, false, r, g, b, a) < 0 {
                            error!("Failed to draw rect");
                        } else {
                            debug!("Drawed rect succesfully");
                        }

                        fbink_wait_for_complete(self.fb.fbfd, LAST_MARKER);
                    }
                }
                Shape::Text(text_shape) => {
                    /*
                    inkview_sys::set_font(
                        self.resource_storage.static_fonts.regular_text_font,
                        Color::rgb(color.r(), color.g(), color.b()),
                    );
                    inkview_sys::draw_text_rect(
                        (pos.x * ppp) as i32,
                        (pos.y * ppp) as i32,
                        (galley.size.x * ppp) as i32,
                        (galley.size.y * ppp) as i32,
                        &*galley.text,
                        inkview_sys::TextAlignFlag::VALIGN_BOTTOM as i32
                            | inkview_sys::TextAlignFlag::ALIGN_LEFT as i32,
                    );
                    */
                    //debug!("text_shape: {:#?}", text_shape);
                    //let got_size = text_shape.galley.job.sections.first().unwrap().format.font_id.size;
                    debug!(
                        "Printing out string: {:?} at pos {:?} with size {:?}",
                        text_shape.galley.text(), text_shape.pos, text_shape.galley.size()
                    );

                    //debug!("galley rect: {:?}", text_shape.galley.rect);

                    unsafe {
                        let mut fbink_ot: FBInkOTConfig =
                            std::mem::transmute([0u8; std::mem::size_of::<FBInkOTConfig>()]);
                        let mut fbink_ot_fit: FBInkOTFit =
                            std::mem::transmute([0u8; std::mem::size_of::<FBInkOTFit>()]);

                        fbink_ot.margins.left = text_shape.pos.x as i16;
                        fbink_ot.margins.top = text_shape.pos.y as i16;
                        //fbink_ot.margins.right = 0;
                        //fbink_ot.margins.bottom = 0;
                        fbink_ot.size_px = text_shape.galley.size().y as u16;
                        let cstr = CString::new(&*text_shape.galley.text()).unwrap();
                        let cchar: *const ::std::os::raw::c_char = cstr.as_ptr();
                        if fbink_print_ot(
                            self.fb.fbfd,
                            cchar,
                            &fbink_ot,
                            &self.fb.fbink_cfg,
                            &mut fbink_ot_fit,
                        ) < 0
                        {
                            error!("Failed to print string");
                        }
                    }
                }
                Shape::Mesh(_) => {}
                Shape::QuadraticBezier(_) => {}
                Shape::CubicBezier(_) => {},
                Shape::Ellipse(_) => {},
                Shape::Callback(_) => {}
            }
        }
    }
    pub fn next_frame(&mut self) {
        let timer = self.egui.get_start_time();

        let raw_input = RawInput {
            screen_rect: Some(Rect {
                min: Pos2 {
                    x: 0.0,
                    y: 0.0,
                },
                max: Pos2 {
                    x: 758.0,
                    y: 1024.0,
                },
            }),
            time: timer.map(|v| v as f64),
            predicted_dt: 1.0/60.0,
            modifiers: Default::default(),
            events: Vec::new(),
            max_texture_side: Some(2048), // Increase this if warnings of texture sizes appear?
            hovered_files: Vec::new(),
            dropped_files: Vec::new(),
            viewport_id: self.egui.view_port_id,
            viewports: self.egui.view_port_list.clone(), // Performance?
            focused: true,
        };

        let mut frame = eframe::Frame {
            info: IntegrationInfo {
                system_theme: None,
                cpu_usage: timer,
            },
            storage: None,
            raw_window_handle: Result::Err(HandleError::NotSupported),
            raw_display_handle: Result::Err(HandleError::NotSupported),
        };

        self.egui.ctx.begin_frame(raw_input);
        self.egui.app.update(&self.egui.ctx, &mut frame);
        let output = self.egui.ctx.end_frame();

        // There is nothing interesting
        // let viewport_out = output.viewport_output.get(&self.egui.view_port_id).unwrap();

        self.draw_shapes(output.shapes);
    }
}
