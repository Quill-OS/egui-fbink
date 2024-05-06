use ::std::os::raw::c_int;
use core::convert::TryInto;
use egui::{
    epaint::{PathShape, RectShape, TextShape},
    Shape,
};
use embedded_graphics::{
    pixelcolor::{
        raw::{RawU16, RawU24},
        Gray8, GrayColor, Rgb555, Rgb565, Rgb888,
    },
    prelude::*,
    primitives::{Circle, PrimitiveStyle, PrimitiveStyleBuilder, Rectangle, RoundedRectangle},
};
use fbink_sys::fbink_get_state;
use fbink_sys::fbink_put_pixel_rgba;
use fbink_sys::fbink_print_ot;
use fbink_sys::fbink_print_raw_data;
use fbink_sys::fbink_put_pixel;
use fbink_sys::FBInkOTConfig;
use fbink_sys::FBInkOTFit;
use fbink_sys::FBInkState;
use fbink_sys::BG_COLOR_INDEX_E_BG_WHITE;
use fbink_sys::FG_COLOR_INDEX_E_FG_WHITE;
use fbink_sys::{
    fbink_add_ot_font, fbink_fill_rect_rgba, fbink_init, fbink_open, fbink_wait_for_complete,
    FBInkConfig, FBInkRect, LAST_MARKER,
};
use ffi::CString;
use log::{debug, error, warn};
use std::{ffi, fs, process::exit};
use fbink_sys::fbink_refresh;

pub struct FBInkBackend {
    pub cfg: FBInkConfig,
    pub fd: c_int,
    pub state: FBInkState,
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
            debug!(
                "Running on {:?}, codename: {:?}, platform: {:?}, with screen: {:?}x{:?}",
                u8_to_string(state.device_name),
                u8_to_string(state.device_codename),
                u8_to_string(state.device_platform),
                state.screen_width,
                state.screen_height
            );

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

        let mut fb = Vec::with_capacity((state.screen_width * state.screen_height) as usize * 3);
        fb.fill(0);

        Self { cfg, fd, state }
    }

    pub fn draw_rect(&mut self, rect: RectShape) {
        let stroke_color = Rgb888::new(
            rect.stroke.color.r(),
            rect.stroke.color.g(),
            rect.stroke.color.b(),
        );
        let fill_color = Rgb888::new(rect.fill.r(), rect.fill.g(), rect.fill.b());

        let style = PrimitiveStyleBuilder::new()
            .stroke_width(rect.stroke.width as u32)
            .stroke_color(stroke_color)
            .fill_color(fill_color)
            .build();

        RoundedRectangle::with_equal_corners(
            Rectangle::new(
                Point::new(rect.rect.left() as i32, rect.rect.top() as i32),
                Size::new(rect.rect.width() as u32, rect.rect.height() as u32),
            ),
            Size::new(rect.rounding.ne as u32, rect.rounding.ne as u32),
        )
        .into_styled(style)
        .draw(self);

        unsafe {
            fbink_refresh(self.fd, rect.rect.left() as u32, rect.rect.top() as u32, rect.rect.width() as u32, rect.rect.height() as u32, &self.cfg);
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

    pub fn draw_paths(&mut self, text: PathShape) {}

    pub fn set_pixel(&self, x: i32, y: i32, color: Rgb888) {
        //debug!("Setting pixel at {}x{} with color {:?}", x, y, color);
        unsafe {

            fbink_put_pixel_rgba(self.fd, x as u16, y as u16, color.r(), color.b(), color.g(), 255);

            /*
            let mut cls_rect: FBInkRect = std::mem::zeroed();
            cls_rect.left = x as u16;
            cls_rect.top = y as u16;
            cls_rect.width = 1;
            cls_rect.height = 1;
            fbink_fill_rect_rgba(self.fd, &self.cfg, &cls_rect, false, color.r(), color.b(), color.g(), 255);
            fbink_wait_for_complete(self.fd, LAST_MARKER);
            */
        }
    }
}

// https://docs.rs/embedded-graphics-core/latest/embedded_graphics_core/draw_target/trait.DrawTarget.html#associatedtype.Color
impl DrawTarget for FBInkBackend {
    type Color = Rgb888;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        let width = self.state.screen_width as i32;
        let height = self.state.screen_height as i32;
        for Pixel(coord, color) in pixels.into_iter() {
            if coord.x < width && coord.y < height && coord.x >= 0 && coord.y >= 0 {
                self.set_pixel(coord.x, coord.y, color);
            }
        }

        Ok(())
    }

    fn fill_solid(&mut self, area: &Rectangle, color: Self::Color) -> Result<(), Self::Error> {
        // Clamp the rectangle coordinates to the valid range by determining
        // the intersection of the fill area and the visible display area
        // by using Rectangle::intersection.
        let area = area.intersection(&self.bounding_box());

        // Do not send a draw rectangle command if the intersection size if zero.
        // The size is checked by using `Rectangle::bottom_right`, which returns `None`
        // if the size is zero.

        let bottom_right = if let Some(bottom_right) = area.bottom_right() {
            bottom_right
        } else {
            return Ok(());
        };

        if area.size.width <= 1 || area.size.height <= 1 {
            //warn!("Using bare pixels to draw this rect: {:?}", area);
            // We need to do it manually because it won't work with rect below 1 px in a direction
            for y in 0..area.size.height {
                for x in 0..area.size.width {
                    self.set_pixel(area.top_left.x + x as i32, area.top_left.y + y as i32, color);
                }
            }
            // We need to refresh this part of it manually because its putting pixels
            // And we can't refresh a single line
            let changer = 1;
            let mut new_width = area.size.width + changer;
            let mut new_height = area.size.height + changer;

            while new_width + area.top_left.x as u32 >= self.state.screen_width {
                new_width = new_width - changer;
            }

            while new_height + area.top_left.y as u32 >= self.state.screen_height {
                new_height = new_height - changer;
            }

            unsafe { fbink_refresh(self.fd, area.top_left.y as u32, area.top_left.x as u32, new_width, new_height, &self.cfg) };
        } else {
            unsafe {
                let mut cls_rect: FBInkRect = std::mem::zeroed();
                cls_rect.left = area.top_left.x as u16;
                cls_rect.top = area.top_left.y as u16;
                cls_rect.width = area.size.width as u16;
                cls_rect.height = area.size.height as u16;
                fbink_fill_rect_rgba(self.fd, &self.cfg, &cls_rect, false, color.r(), color.g(), color.b(), 255);
                fbink_wait_for_complete(self.fd, LAST_MARKER);
            }
        }

        Ok(())
    }


    type Error = core::convert::Infallible;
}

impl OriginDimensions for FBInkBackend {
    fn size(&self) -> Size {
        Size::new(self.state.screen_width, self.state.screen_height)
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
