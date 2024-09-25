#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(clippy::new_without_default, clippy::type_complexity)]

mod cmd;
mod controller;
mod data;
mod delegate;
mod error;
mod ui;
mod webapi;
mod widget;

use druid::AppLauncher;
use env_logger::{Builder, Env};
use webapi::WebApi;
use std::env;

use crate::{
    data::{AppState, Config},
    delegate::Delegate,
};

const ENV_LOG: &str = "PSST_LOG";
const ENV_LOG_STYLE: &str = "PSST_LOG_STYLE";

fn main() {
    // Setup logging from the env variables, with defaults.
    Builder::from_env(
        Env::new()
            .filter_or(ENV_LOG, "info")
            .write_style(ENV_LOG_STYLE),
    )
    .init();

    let config = Config::load().unwrap_or_default();
    let paginated_limit = config.paginated_limit;
    let mut state = AppState::default_with_config(config);

    let args: Vec<String> = env::args().collect();
    state.config.kiosk_mode = false;
    if !args.is_empty() && (args.contains(&"-k".to_string()) || args.contains(&"-kiosk".to_string())) {
        state.config.kiosk_mode = true;
    }

    WebApi::new(
        state.session.clone(),
        Config::proxy().as_deref(),
        Config::cache_dir(),
        paginated_limit,
    )
    .install_as_global();

    let delegate;
    let launcher;
    if state.config.has_credentials() {
        // Credentials are configured, open the main window.
        let window = ui::main_window(&state.config);
        delegate = Delegate::with_main(window.id);
        launcher = AppLauncher::with_window(window).configure_env(ui::theme::setup);

        // Load user's local tracks for the WebApi.
        WebApi::global().load_local_tracks(state.config.username().unwrap());
    } else {
        // No configured credentials, open the account setup.
        let mut window = ui::account_setup_window();
        
        if state.config.kiosk_mode {
            window = ui::kiosk_setup_window();
        } 

        delegate = Delegate::with_preferences(window.id);
        launcher = AppLauncher::with_window(window).configure_env(ui::theme::setup);
    };

    launcher
        .delegate(delegate)
        .launch(state)
        .expect("Application launch");
}
