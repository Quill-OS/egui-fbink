use ::std::os::raw::c_int;
use egui::{
    epaint::{RectShape, TextShape},
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
use raqote::{DrawOptions, DrawTarget, SolidSource, Source};
use std::{ffi, fs, process::exit};

pub struct FBInkBackend {
    pub fbink_cfg: FBInkConfig,
    pub fbfd: c_int,
}

impl FBInkBackend {
    pub fn new() -> Self {
        let fbfd: c_int = unsafe { fbink_open() };
        if fbfd < 0 {
            error!("Failed to open fbink");
            exit(1);
        }
        let mut fbink_cfg: FBInkConfig = unsafe { std::mem::zeroed() };

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

            let mut cls_rect: FBInkRect = std::mem::zeroed();
            cls_rect.left = 0;
            cls_rect.top = 0;
            cls_rect.width = 0;
            cls_rect.height = 0;
            fbink_fill_rect_rgba(fbfd, &fbink_cfg, &cls_rect, false, 255, 255, 255, 255);
            fbink_wait_for_complete(fbfd, LAST_MARKER);
        }

        Self { fbink_cfg, fbfd }
    }

    pub fn drawRect(&mut self, rect: RectShape) {
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
        let mut dt = DrawTarget::new(width, height);
        dt.fill_rect(
            0.0,
            0.0,
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
        let rdata = dt.get_data_u8();
        let data: *const u8 = rdata.as_ptr();
        unsafe {
            fbink_print_raw_data(
                self.fbfd,
                data,
                width,
                height,
                rdata.len(),
                rect.rect.min.x as i16,
                rect.rect.min.y as i16,
                &self.fbink_cfg,
            );
            fbink_wait_for_complete(self.fbfd, LAST_MARKER);
        }
    }

    pub fn drawText(&mut self, text: TextShape) {
        unsafe {
            let mut fbink_ot: FBInkOTConfig = std::mem::zeroed();
            let mut fbink_ot_fit: FBInkOTFit = std::mem::zeroed();
            let mut font_fb_config = self.fbink_cfg;
            font_fb_config.fg_color = 255;
            font_fb_config.bg_color = 255;
    
            fbink_ot.margins.left = text.pos.x as i16;
            fbink_ot.margins.top = text.pos.y as i16;
            //fbink_ot.margins.right = 0;
            //fbink_ot.margins.bottom = 0;
            fbink_ot.size_px = text.galley.size().y as u16;
            let cstr = CString::new(&*text.galley.text()).unwrap();
            let cchar: *const ::std::os::raw::c_char = cstr.as_ptr();
            if fbink_print_ot(
                self.fbfd,
                cchar,
                &fbink_ot,
                &font_fb_config,
                &mut fbink_ot_fit,
            ) < 0
            {
                error!("Failed to print string");
            }
            fbink_wait_for_complete(self.fbfd, LAST_MARKER);
        }
    }
}
