use std::{collections::HashMap, hash::BuildHasherDefault};

use eframe::App;
use egui::{Context, ViewportId, ViewportInfo};

pub struct EguiStuff {
    pub ctx: Context,
    pub app: Box<dyn App>,
    pub pixel_per_point: f32,
    pub view_port_id: ViewportId,
    pub view_port_list: HashMap<ViewportId, ViewportInfo, BuildHasherDefault<nohash_hasher::NoHashHasher<ViewportId>>>,

}
impl EguiStuff {
    pub fn new(app: Box<dyn App>, pixel_per_point: f32) -> Self {
        let ctx = Context::default();
        // debug!("Default pixels per point: {}", ctx.pixels_per_point()); // It's always 1
        ctx.set_pixels_per_point(pixel_per_point);

        let mut view_port_list: HashMap<ViewportId, ViewportInfo, BuildHasherDefault<nohash_hasher::NoHashHasher<ViewportId>>> = Default::default();
        let view_port_id = ViewportId::default();
        view_port_list.insert(view_port_id, ViewportInfo::default());

        Self { 
            ctx,
            app,
            pixel_per_point,
            view_port_id,
            view_port_list,
        }
    }
}
