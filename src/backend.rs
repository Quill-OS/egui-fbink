use ::std::os::raw::c_int;
use egui::epaint::ClippedShape;
use egui::Event;
use egui::{CtxRef, PointerButton, Pos2, Shape, TouchDeviceId, TouchId, TouchPhase, Vec2};
use epi::{IntegrationInfo, Storage};
use fbink_sys::*;
use log::{debug, error};
use std::process::exit;
use std::sync::atomic::Ordering::SeqCst;
use std::sync::Arc;

use crate::texture_allocator::FbinkTextureAllocator;

pub struct FbinkBackend {
    pub egui_ctx: CtxRef,
    previous_frame_time: Option<f32>,
    frame_start: Option<f64>,
    fbink_cfg: FBInkConfig,
    fbfd: c_int,
}
impl FbinkBackend {
    pub(crate) fn new() -> Self {
        let ctx = CtxRef::default();
        let fbfd: c_int = unsafe { fbink_open() };
        if fbfd < 0 {
            error!("Failed to open fbink");
            exit(1);
        }
        let mut fbink_cfg: FBInkConfig =
            unsafe { std::mem::transmute([0u8; std::mem::size_of::<FBInkConfig>()]) };

        unsafe {
            if fbink_init(fbfd, &fbink_cfg) < 0 {
                error!("Failed to init fbink");
                exit(1);
            }
        }

        Self {
            egui_ctx: ctx,
            previous_frame_time: None,
            frame_start: None,
            fbink_cfg,
            fbfd,
        }
    }
    pub fn begin_frame(&mut self, raw_input: egui::RawInput) {
        self.frame_start = Some(1f64); // TODO // My god what is this
        self.egui_ctx.begin_frame(raw_input)
    }

    pub fn end_frame(&mut self) -> (egui::Output, Vec<ClippedShape>) {
        let frame_start = self
            .frame_start
            .take()
            .expect("unmatched calls to begin_frame/end_frame");

        let (output, shapes) = self.egui_ctx.end_frame();

        //let clipped_meshes = self.egui_ctx.tessellate(shapes);

        let now = 1f64; // TODO
        self.previous_frame_time = Some((now - frame_start) as f32);

        (output, shapes)
    }
}

pub struct AppRunner {
    fbink_backend: FbinkBackend,
    app: Box<dyn epi::App>,
    pub(crate) needs_repaint: std::sync::Arc<NeedRepaint>,
    //resource_storage: ResourceStorage
}

pub struct NeedRepaint(std::sync::atomic::AtomicBool);

impl Default for NeedRepaint {
    fn default() -> Self {
        Self(true.into())
    }
}

impl NeedRepaint {
    pub fn fetch_and_clear(&self) -> bool {
        self.0.swap(false, SeqCst)
    }

    pub fn set_true(&self) {
        self.0.store(true, SeqCst);
    }
}

impl epi::RepaintSignal for NeedRepaint {
    fn request_repaint(&self) {
        self.0.store(true, SeqCst);
    }
}

impl AppRunner {
    pub(crate) fn new(fbink_backend: FbinkBackend, app: Box<dyn epi::App>) -> Self {
        fbink_backend.egui_ctx.set_visuals(egui::Visuals::light());
        let mut runner = Self {
            fbink_backend,
            app,
            needs_repaint: Arc::from(NeedRepaint::default()),
            //resource_storage: ResourceStorage::new(),
        };

        let mut app_output = epi::backend::AppOutput::default();
        let mut texture_allocator = FbinkTextureAllocator {};
        let mut frame = epi::backend::FrameBuilder {
            info: IntegrationInfo {
                web_info: None,
                prefer_dark_mode: None,
                cpu_usage: None,
                seconds_since_midnight: None,
                native_pixels_per_point: Some(3.0),
            },
            tex_allocator: &mut texture_allocator,
            #[cfg(feature = "http")]
            http: runner.http.clone(),
            output: &mut app_output,
            repaint_signal: runner.needs_repaint.clone(),
        }
        .build();

        runner
            .app
            .setup(&runner.fbink_backend.egui_ctx, &mut frame, None);

        runner
    }
    pub fn draw_shapes(&mut self, clipped_shapes: Vec<ClippedShape>) {
        let ppp = self.fbink_backend.egui_ctx.pixels_per_point();
        for ClippedShape(clip_rect, shape) in clipped_shapes {
            if !clip_rect.is_positive() {
                continue;
            }

            match shape {
                Shape::Noop => {}
                Shape::Vec(_) => {}
                Shape::Circle {
                    center,
                    radius,
                    fill,
                    stroke,
                } => {
                    /*
                    inkview_sys::draw_circle(
                        (center.x * ppp) as i32,
                        (center.y * ppp) as i32,
                        (radius * ppp) as i32,
                        inkview_sys::Color::rgb(fill.r(), fill.g(), fill.b()),
                    );
                    */
                }
                Shape::LineSegment { .. } => {}
                Shape::Path { .. } => {}
                Shape::Rect {
                    rect,
                    corner_radius,
                    fill,
                    stroke,
                } => {
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

                    debug!("Printing out rectangle at {:?}", rect);
                    let fbink_rect: FBInkRect = FBInkRect {
                        left: rect.left() as u16,
                        top: rect.top() as u16,
                        width: rect.width() as u16,
                        height: rect.height() as u16,
                    };

                    unsafe {
                        if fbink_cls(self.fbink_backend.fbfd, &self.fbink_backend.fbink_cfg, &fbink_rect, false) < 0 {
                            error!("Failed to draw rect");
                        } else {
                            debug!("Drawed rect succesfully");
                        }

                        fbink_wait_for_complete(self.fbink_backend.fbfd, LAST_MARKER);
                    }
                }
                Shape::Text {
                    pos,
                    galley,
                    color,
                    fake_italics,
                } => {
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
                    debug!("Printing out string: {}", &*galley.text);
                }
                Shape::Mesh(_) => {}
            }
        }
    }
    pub fn next_frame(&mut self) {
        let raw_input: egui::RawInput = egui::RawInput {
            scroll_delta: Vec2 { x: 0.0, y: 0.0 },
            zoom_delta: 0.0,
            screen_size: Vec2 {
                x: 758f32 / 2f32,
                y: 1024f32 / 2f32,
            },
            screen_rect: Some(egui::Rect {
                min: Default::default(),
                max: egui::Pos2 {
                    x: 758f32,
                    y: 1024f32,
                },
            }),
            pixels_per_point: Some(3f32),
            time: None,
            predicted_dt: 0.0,
            modifiers: Default::default(),
            events: Vec::new(),
        };
        self.fbink_backend.begin_frame(raw_input);

        let mut texture_allocator = FbinkTextureAllocator {};

        let mut app_output = epi::backend::AppOutput::default();

        let mut frame = epi::backend::FrameBuilder {
            info: IntegrationInfo {
                web_info: None,
                prefer_dark_mode: None,
                cpu_usage: None,
                seconds_since_midnight: None,
                native_pixels_per_point: None,
            },
            tex_allocator: &mut texture_allocator,
            #[cfg(feature = "http")]
            http: runner.http.clone(),
            output: &mut app_output,
            repaint_signal: self.needs_repaint.clone(),
        }
        .build();

        self.app.update(&self.fbink_backend.egui_ctx, &mut frame);

        let (output, shapes) = self.fbink_backend.end_frame();
        self.draw_shapes(shapes);
    }
}