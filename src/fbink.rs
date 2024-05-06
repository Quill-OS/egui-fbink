use ::std::os::raw::c_int;
use egui::{
    epaint::{PathShape, RectShape, TextShape},
    Shape,
};
use fbink_sys::fbink_print_ot;
use fbink_sys::fbink_print_raw_data;
use fbink_sys::FBInkOTConfig;
use fbink_sys::FBInkOTFit;
use fbink_sys::{
    fbink_add_ot_font, fbink_fill_rect_rgba, fbink_init, fbink_open, fbink_wait_for_complete,
    FBInkConfig, FBInkRect, LAST_MARKER,
};
use ffi::CString;
use log::{debug, error};
use raqote::{DrawOptions, DrawTarget, IntPoint, IntRect, SolidSource, Source};
use std::{ffi, fs, process::exit};
use fbink_sys::BG_COLOR_INDEX_E_BG_WHITE;
use fbink_sys::FG_COLOR_INDEX_E_FG_WHITE;
use fbink_sys::FBInkState;
use fbink_sys::fbink_get_state;
use euclid::Point2D;

pub struct FBInkBackend {
    pub cfg: FBInkConfig,
    pub fd: c_int,
    pub state: FBInkState,
    pub dt: DrawTarget,
}

impl FBInkBackend {
    pub fn new() -> Self {
        let fd: c_int = unsafe { fbink_open() };
        if fd < 0 {
            error!("Failed to open fbink");
            exit(1);
        }
        let mut cfg: FBInkConfig = unsafe { std::mem::zeroed() };

        let mut state: FBInkState = unsafe { std::mem::zeroed() };

        unsafe {
            if fbink_init(fd, &cfg) < 0 {
                error!("Failed to init fbink");
                exit(1);
            }

            fbink_get_state(&cfg, &mut state);
            // Why does it compile but it shows errors
            debug!("Running on {:?}, codename: {:?}, platform: {:?}, with screen: {:?}x{:?}", u8_to_string(state.device_name), u8_to_string(state.device_codename), u8_to_string(state.device_platform), state.screen_width, state.screen_height);

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

            let mut cls_rect: FBInkRect = std::mem::zeroed();
            cls_rect.left = 0;
            cls_rect.top = 0;
            cls_rect.width = 0;
            cls_rect.height = 0;
            fbink_fill_rect_rgba(fd, &cfg, &cls_rect, false, 255, 255, 255, 255);
            fbink_wait_for_complete(fd, LAST_MARKER);
        }

        let mut dt = DrawTarget::new(state.screen_width as i32, state.screen_height as i32);

        Self { cfg, fd, state, dt }
    }

    pub fn draw_rect(&mut self, rect: RectShape) {
        /*
        let fbink_rect: FBInkRect = FBInkRect {
            left: rect.rect.left() as u16,
            top: rect.rect.top() as u16,
            width: rect.rect.width() as u16,
            height: rect.rect.height() as u16,
        };
        let r: u8 = rect.fill.r();
        let g: u8 = rect.fill.g();
        let b: u8 = rect.fill.b();
        let a: u8 = rect.fill.a();

        unsafe {
            if fbink_fill_rect_rgba(self.fbfd, &self.fbink_cfg, &fbink_rect, false, r, g, b, a) < 0
            {
                error!("Failed to draw rect");
            } else {
                debug!("Drawed rect succesfully");
            }

            fbink_wait_for_complete(self.fbfd, LAST_MARKER);
        }
        */
        let height = rect.rect.height() as i32;
        let width = rect.rect.width() as i32;
        self.dt.fill_rect(
            rect.rect.left(),
            rect.rect.top(),
            rect.rect.width(),
            rect.rect.height(),
            &Source::Solid(SolidSource::from_unpremultiplied_argb(
                rect.fill.a(),
                rect.fill.r(),
                rect.fill.g(),
                rect.fill.b(),
            )),
            &DrawOptions::new(),
        );

        let data: *const u8 = rdata.as_ptr();
        unsafe {
            fbink_print_raw_data(
                self.fd,
                data,
                width,
                height,
                rdata.len(),
                rect.rect.min.x as i16,
                rect.rect.min.y as i16,
                &self.cfg,
            );
            fbink_wait_for_complete(self.fd, LAST_MARKER);
        }
    }

    pub fn draw_text(&mut self, text: TextShape) {
        unsafe {
            let mut fbink_ot: FBInkOTConfig = std::mem::zeroed();
            let mut fbink_ot_fit: FBInkOTFit = std::mem::zeroed();
            let mut font_fb_config: FBInkConfig = std::mem::zeroed(); // self.fbink_cfg;
            //font_fb_config.pen_fg_color = FG_COLOR_INDEX_E_FG_WHITE;
            //font_fb_config.pen_bg_color = BG_COLOR_INDEX_E_BG_WHITE;
    
            fbink_ot.margins.left = text.pos.x as i16;
            fbink_ot.margins.top = text.pos.y as i16;
            //fbink_ot.margins.right = 0;
            //fbink_ot.margins.bottom = 0;
            fbink_ot.size_px = text.galley.size().y as u16;
            let cstr = CString::new(&*text.galley.text()).unwrap();
            let cchar: *const ::std::os::raw::c_char = cstr.as_ptr();
            if fbink_print_ot(
                self.fd,
                cchar,
                &fbink_ot,
                &font_fb_config,
                &mut fbink_ot_fit,
            ) < 0
            {
                error!("Failed to print string");
            }
            fbink_wait_for_complete(self.fd, LAST_MARKER);
        }
    }

    pub fn draw_paths(&mut self, text: PathShape) {

    }
}

pub fn rgb_to_gray(r: u8, g: u8, b: u8) -> u8 {
    // 709 formula
    let gray = (0.2126 * (r as f32) + 0.7152 * (g as f32) + 0.0722 * (b as f32)).round() as u8;
    gray
}

pub fn invert_byte(b: u8) -> u8 {
    !b
}

fn u8_to_string(arr: [u8; 32]) -> String {
    let mut str: String = arr.iter().map(|&c| c as char).collect();
    str.replace("\0", "")
}