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
    pub(crate) fn new(egui: EguiStuff, fb: FBInkBackend) -> Self {
        let mut runner = Self {
            fb,
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
        //let ppp = self.egui.pixel_per_point;
        //debug!("draw shapes pixel per point: {}", ppp);
        for shape in clipped_shapes {
            if shape.clip_rect.is_negative() {
                error!("clip rect is negative");
                continue
            } else {
                // debug!("shape.0: {:?}", shape.0);
            }
            match shape.shape {
                Shape::Noop => {}
                Shape::Vec(vec) => {
                    debug!("Printing out vecs: {:?}", vec);
                }
                Shape::Circle(circle) => {
                    debug!("Printing out circles: {:?}", circle);
                }
                Shape::LineSegment {points, stroke} => {
                    debug!("Printing out points {:?} with strokes {:?}", points, stroke);
                }
                Shape::Path(path) => {
                    debug!("Printing out path: {:?}", path);
                    self.fb.draw_paths(path);
                }
                Shape::Rect(rect) => {
                    debug!("Printing out rectangle at {:?}", rect);
                    self.fb.draw_rect(rect);
                }
                Shape::Text(text) => {
                    // debug!(
                    //     "Printing out string: {:?} at pos {:?} with size {:?}",
                    //     text.galley.text(), text.pos, text.galley.size()
                    // );
                    self.fb.draw_text(text);
                }
                Shape::Mesh(mesg) => {}
                Shape::QuadraticBezier(qb) => {}
                Shape::CubicBezier(cb) => {},
                Shape::Ellipse(ellipse) => {},
                Shape::Callback(callback) => {}
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
                    x: self.fb.state.screen_width as f32,
                    y: self.fb.state.screen_height as f32,
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

        // in /home/szybet/.cargo/registry/src/index.crates.io-6f17d22bba15001f/egui-0.27.2/src/context.rs
        // ContextImpl::begin_frame_mut
        // if let Some(new_zoom_factor) = self.new_zoom_factor.take()
        // to
        // if let Some(new_zoom_factor) = self.new_zoom_factor
        /*
        // Doesn't really work
        if timer.is_none() {
            self.egui.ctx.set_zoom_factor(self.egui.zoom_factor);
            debug!("Setting initial zoom factor");
        }

        self.egui.ctx.request_repaint();
        */

        self.egui.app.update(&self.egui.ctx, &mut frame);

        let output = self.egui.ctx.end_frame();

        // There is nothing interesting
        // let viewport_out = output.viewport_output.get(&self.egui.view_port_id).unwrap();

        self.draw_shapes(output.shapes);
    }
}
