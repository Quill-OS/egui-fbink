use eframe::NativeOptions;
use egui_fbink;
use std::{panic, thread};
use std::fs::File;
use std::io::Write;
use std::ffi::CString;
use core::time;
use std::sync::{Mutex, Arc};
use backtrace::Backtrace;
use log::debug;

mod app;
use crate::app::TemplateApp;

fn main() {
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "debug"),
    );
    debug!("Starting the app");
    let app = TemplateApp::default();
    let mut native_options = NativeOptions::default();
    native_options.hardware_acceleration = eframe::HardwareAcceleration::Off;
    native_options.vsync = false;
    egui_fbink::start(Box::new(app), native_options, 1.0, 3.5);
}