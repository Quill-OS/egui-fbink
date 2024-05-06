use std::{collections::HashMap, hash::BuildHasherDefault, time::SystemTime};

use eframe::App;
use egui::FontFamily::Proportional;
use egui::FontId;
use egui::TextStyle::*;
use egui::{Context, Pos2, Rect, Vec2, ViewportId, ViewportInfo};

use crate::fbink::FBInkBackend;

pub struct EguiStuff {
    pub ctx: Context,
    pub app: Box<dyn App>,
    pub pixel_per_point: f32,
    pub zoom_factor: f32,
    pub view_port_id: ViewportId,
    pub view_port_list: HashMap<
        ViewportId,
        ViewportInfo,
        BuildHasherDefault<nohash_hasher::NoHashHasher<ViewportId>>,
    >,
    pub start_time: Option<SystemTime>,
}
impl EguiStuff {
    pub fn new(
        app: Box<dyn App>,
        fb: &FBInkBackend,
        pixel_per_point: f32,
        zoom_factor: f32,
    ) -> Self {
        let ctx = Context::default();
        ctx.set_embed_viewports(true);
        ctx.set_pixels_per_point(pixel_per_point);
        //ctx.set_visuals(egui::Visuals::light());

        let mut style = (*ctx.style()).clone();

        for ts in &mut style.text_styles {
            ts.1.size *= zoom_factor;
        }

        style.spacing.icon_width_inner *= zoom_factor;
        style.spacing.icon_width *= zoom_factor;

        ctx.set_style(style);

        /*
        let mut fonts = egui::FontDefinitions::default();
        fonts.font_data.insert(
            "my_font".to_owned(),
            egui::FontData::from_static(include_bytes!(
                "/mnt/data/projects/inkbox/egui-fbink/Ubuntu-Light.ttf"
            )),
        ); // .ttf and .otf supported

        fonts
            .families
            .get_mut(&egui::FontFamily::Proportional)
            .unwrap()
            .insert(0, "my_font".to_owned());

        ctx.set_fonts(fonts);
        */

        let mut view_port_list: HashMap<
            ViewportId,
            ViewportInfo,
            BuildHasherDefault<nohash_hasher::NoHashHasher<ViewportId>>,
        > = Default::default();
        let view_port_id = ViewportId::default();
        let mut view_port_info = ViewportInfo::default();

        let screen_size = Some(Vec2 {
            x: fb.state.screen_width as f32,
            y: fb.state.screen_height as f32,
        });

        let screen_size_rect = Some(Rect {
            min: Pos2 { x: 0.0, y: 0.0 },
            max: Pos2 {
                x: fb.state.screen_width as f32,
                y: fb.state.screen_height as f32,
            },
        });

        view_port_info.native_pixels_per_point = Some(pixel_per_point);
        view_port_info.monitor_size = screen_size;
        view_port_info.inner_rect = screen_size_rect;
        view_port_info.outer_rect = screen_size_rect;
        view_port_info.fullscreen = Some(true);
        view_port_info.focused = Some(true);

        view_port_list.insert(view_port_id, view_port_info);

        Self {
            ctx,
            app,
            pixel_per_point,
            zoom_factor,
            view_port_id,
            view_port_list,
            start_time: None,
        }
    }

    // I don't thing this is needed at all
    pub fn get_start_time(&mut self) -> Option<f32> {
        if let Some(start_time) = self.start_time {
            return Some(
                start_time
                    .elapsed()
                    .expect("Failed to get elapsed time")
                    .as_secs_f32(),
            );
        } else {
            self.start_time = Some(SystemTime::now());
            return None;
        }
    }
}
