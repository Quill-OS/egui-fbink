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

static pixel_per_point: f32 = 10.0;

pub struct FbinkBackend {
    pub egui_ctx: Context,
    fbink_cfg: FBInkConfig,
    fbfd: c_int,
}
impl FbinkBackend {
    pub(crate) fn new() -> Self {
        let ctx = Context::default();
        debug!("Default pixels per point: {}", ctx.pixels_per_point());
        ctx.set_pixels_per_point(pixel_per_point);

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

            static FONT_PATH: &str = "fonts/";
            for entry in fs::read_dir(FONT_PATH).expect("fonts dir wasn't found") {
                let real_entry = entry.unwrap();
                let name = format!(
                    "{}{}",
                    FONT_PATH,
                    real_entry.file_name().to_string_lossy().to_string()
                );
                if real_entry.file_type().unwrap().is_file() {
                    debug!("Adding OT font: {}", name);
                }
                let c_text = CString::new(name.clone()).unwrap();
                let c_chat = c_text.as_ptr();
                // 0 as regular?
                if fbink_add_ot_font(c_chat, 0) < 0 {
                    error!("Failed to add font: {}", name);
                }
            }

            let mut cls_rect: FBInkRect =
                std::mem::transmute([0u8; std::mem::size_of::<FBInkRect>()]);
            cls_rect.left = 0;
            cls_rect.top = 0;
            cls_rect.width = 0; 
            cls_rect.height = 0;
            fbink_cls(fbfd, &fbink_cfg, &cls_rect, false);
            fbink_wait_for_complete(fbfd, LAST_MARKER);
        }

        Self {
            egui_ctx: ctx,
            fbink_cfg,
            fbfd,
        }
    }

    pub fn begin_frame(&mut self, raw_input: RawInput) {
        self.egui_ctx.begin_frame(raw_input)
    }

    pub fn end_frame(&mut self) -> FullOutput {
        let mut output: FullOutput = self.egui_ctx.end_frame();
        output
    }
}

pub struct AppRunner {
    fbink_backend: FbinkBackend,
    app: Box<dyn App>,
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

// Gone?
/*
impl epi::backend::RepaintSignal for NeedRepaint {
    fn request_repaint(&self) {
        self.0.store(true, SeqCst);
    }
}
*/

impl AppRunner {
    pub(crate) fn new(fbink_backend: FbinkBackend, app: Box<dyn App>) -> Self {
        fbink_backend.egui_ctx.set_visuals(egui::Visuals::dark());
        let mut runner = Self {
            fbink_backend,
            app,
            needs_repaint: Arc::from(NeedRepaint::default()),
            //resource_storage: ResourceStorage::new(),
        };

        /*
        let mut app_output = output::FullOutput::default();
        let mut frame = Frame {
            info: IntegrationInfo {
                name: "egui_fbink",
                web_info: None,
                prefer_dark_mode: None,
                cpu_usage: None,
                native_pixels_per_point: Some(pixel_per_point),
            },
            #[cfg(feature = "http")]
            http: runner.http.clone(),
            output: app_output,
            repaint_signal: runner.needs_repaint.clone(),
        };
        */
        // Gone?
        /*
        runner
            .app
            .setup(&runner.fbink_backend.egui_ctx, &frame_data, None);
        */

        runner
    }
    pub fn draw_shapes(&mut self, clipped_shapes: Vec<ClippedShape>) {
        let ppp = self.fbink_backend.egui_ctx.pixels_per_point();

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
                        if fbink_fill_rect_rgba(self.fbink_backend.fbfd, &self.fbink_backend.fbink_cfg, &fbink_rect, false, r, g, b, a) < 0 {
                            error!("Failed to draw rect");
                        } else {
                            debug!("Drawed rect succesfully");
                        }

                        fbink_wait_for_complete(self.fbink_backend.fbfd, LAST_MARKER);
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
                            self.fbink_backend.fbfd,
                            cchar,
                            &fbink_ot,
                            &self.fbink_backend.fbink_cfg,
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
        let mut view_port_list: HashMap<ViewportId, ViewportInfo, BuildHasherDefault<nohash_hasher::NoHashHasher<ViewportId>>> = Default::default();
        let view_port_id = ViewportId::default();
        let view_port_info = ViewportInfo::default();
        view_port_list.insert(view_port_id, view_port_info);

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
            time: None,
            predicted_dt: 1.0/60.0,
            modifiers: Default::default(),
            events: Vec::new(),
            max_texture_side: None,
            hovered_files: Vec::new(),
            dropped_files: Vec::new(),
            viewport_id: view_port_id,
            viewports: view_port_list,
            focused: true,
        };
        self.fbink_backend.begin_frame(raw_input);

        //let mut app_output = AppOutput::default();
        let int_info = IntegrationInfo {
            system_theme: None,
            cpu_usage: None,
        };
        
        let mut frame = eframe::Frame {
            info: int_info,
            storage: None,
            raw_window_handle: Result::Err(HandleError::NotSupported),
            raw_display_handle: Result::Err(HandleError::NotSupported),
        };

        // self.fbink_backend.egui_ctx.request_repaint(); // idk
        self.app.update(&self.fbink_backend.egui_ctx, &mut frame);
        
        let output = self.fbink_backend.end_frame();
        /*
        let whatnow = self.fbink_backend.egui_ctx.tessellate(output.shapes.clone());
        for what in whatnow {
            debug!("whatnow: {:?}", what.0);
            for vertexes in what.1.vertices {
                debug!("whatnext: {:?}", vertexes.pos);
            }
        }
        // in
        // /home/szybet/.cargo/registry/src/index.crates.io-6f17d22bba15001f/epaint-0.17.0/src/tessellator.rs
        // function tessellate_shape
        // Shape::Text
        */
        self.draw_shapes(output.shapes);
    }
}
