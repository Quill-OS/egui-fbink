use std::{collections::HashMap, hash::BuildHasherDefault, time::SystemTime};

use eframe::App;
use egui::style::WidgetVisuals;
use egui::FontFamily::Proportional;
use egui::FontId;
use egui::Rounding;
use egui::TextStyle::*;
use egui::{Context, Pos2, Rect, Vec2, ViewportId, ViewportInfo};

use crate::eink_theme::style;
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
        ctx.set_visuals(egui::Visuals::light());
        ctx.set_style(style()); // Set the eink style

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

    pub fn manage_zoom(&mut self) {
        let mut style = (*self.ctx.style()).clone();

        for ts in &mut style.text_styles {
            ts.1.size *= self.zoom_factor;
        }

        let rounding_zoom = |rounding: &mut Rounding| {
            rounding.nw *= self.zoom_factor;
            rounding.ne *= self.zoom_factor;
            rounding.sw *= self.zoom_factor;
            rounding.se *= self.zoom_factor;
        };

        let widget_visual_zoom = |widget_visuals: &mut WidgetVisuals| {
            rounding_zoom(&mut widget_visuals.rounding);
    
            widget_visuals.bg_stroke.width *= self.zoom_factor;
            widget_visuals.fg_stroke.width *= self.zoom_factor;
    
            widget_visuals.expansion *= self.zoom_factor;
        };

        // Adjustments in the `Spacing` struct
        style.spacing.item_spacing *= self.zoom_factor;
        style.spacing.button_padding *= self.zoom_factor;
        style.spacing.menu_margin.left *= self.zoom_factor;
        style.spacing.menu_margin.right *= self.zoom_factor;
        style.spacing.window_margin.top *= self.zoom_factor;
        style.spacing.window_margin.bottom *= self.zoom_factor;
        style.spacing.indent *= self.zoom_factor;
        style.spacing.interact_size *= self.zoom_factor;
        style.spacing.slider_width *= self.zoom_factor;
        style.spacing.slider_rail_height *= self.zoom_factor;
        style.spacing.combo_width *= self.zoom_factor;
        style.spacing.text_edit_width *= self.zoom_factor;
        style.spacing.icon_width *= self.zoom_factor;
        style.spacing.icon_width_inner *= self.zoom_factor;
        style.spacing.icon_spacing *= self.zoom_factor;
        style.spacing.tooltip_width *= self.zoom_factor;
        style.spacing.menu_width *= self.zoom_factor;
        style.spacing.menu_spacing *= self.zoom_factor;
        style.spacing.combo_height *= self.zoom_factor;

        // Adjustments in the `Interaction` struct
        style.interaction.interact_radius *= self.zoom_factor;
        style.interaction.resize_grab_radius_side *= self.zoom_factor;
        style.interaction.resize_grab_radius_corner *= self.zoom_factor;
        style.interaction.tooltip_delay *= self.zoom_factor;

        // Adjustments in the `Visuals` struct
        rounding_zoom(&mut style.visuals.window_rounding);
                
        style.visuals.window_shadow.offset *= self.zoom_factor;

        rounding_zoom(&mut style.visuals.menu_rounding);

        style.visuals.resize_corner_size *= self.zoom_factor;
        style.visuals.text_cursor.width *= self.zoom_factor;
        style.visuals.clip_rect_margin *= self.zoom_factor;

        widget_visual_zoom(&mut style.visuals.widgets.noninteractive);
        widget_visual_zoom(&mut style.visuals.widgets.inactive);
        widget_visual_zoom(&mut style.visuals.widgets.hovered);
        widget_visual_zoom(&mut style.visuals.widgets.active);
        widget_visual_zoom(&mut style.visuals.widgets.open);

        self.ctx.set_style(style);
    }
}
