use ffi::CString;
use fbink_sys::{fbink_add_ot_font, fbink_fill_rect_rgba, fbink_init, fbink_open, fbink_wait_for_complete, FBInkConfig, FBInkRect, LAST_MARKER};
use log::{debug, error};
use ::std::os::raw::c_int;
use std::{ffi, fs, process::exit};

pub struct FBInkBackend {
    pub fbink_cfg: FBInkConfig,
    pub fbfd: c_int,
}

impl FBInkBackend {
    pub(crate) fn new() -> Self {
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
            fbink_fill_rect_rgba(fbfd, &fbink_cfg, &cls_rect, false, 255, 255, 255, 255);
            fbink_wait_for_complete(fbfd, LAST_MARKER);
        }

        Self {
            fbink_cfg,
            fbfd,
        }
    }
}