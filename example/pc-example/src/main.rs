use epi::NativeOptions;
use std::{panic, thread};
use std::fs::File;
use std::io::Write;
use std::ffi::CString;
use core::time;
use std::sync::{Mutex, Arc};
use backtrace::Backtrace;
use log::debug;
use eframe::egui;

mod app;
use crate::app::TemplateApp;

fn main() {
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "debug"),
    );
    debug!("Starting the app");

    let app = TemplateApp::default();
    let options = eframe::NativeOptions {
        ..Default::default()
    };

    eframe::run_native(
        Box::new(app), options
    )
}