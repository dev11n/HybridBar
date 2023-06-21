use crate::{config::Config, constants::*, widgets::cava_widget::CavaWidget};
use smallvec::SmallVec;
use std::{
    fs::write,
    process::Stdio,
    sync::{Mutex, RwLock},
};
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    process::Command,
    task,
};

lazy_static! {
    /// Has Cava been started yet?
    pub static ref HAS_CAVA_STARTED: Mutex<bool> = Mutex::new(false);
    /// Current Cava bars.
    pub static ref BARS: RwLock<String> = RwLock::new(String::default());
    /// Has Cava crashed? If true, don't keep `update_cava` running.
    pub static ref HAS_CAVA_CRASHED: RwLock<bool> = RwLock::new(false);
    /// All active Cava widget instances.
    pub static ref CAVA_INSTANCES: RwLock<SmallVec<[CavaWidget; 2]>> = RwLock::new(SmallVec::new());
}

/// Gets the sed to use for Cava.
pub fn get_sed(config: &'static Config) -> String {
    config.read_config_raw()[HYBRID_ROOT_JSON]["cava_sed"]
        .as_str()
        .unwrap_or("s/;//g;s/0/▁/g;s/1/▂/g;s/2/▃/g;s/3/▄/g;s/4/▅/g;s/5/▆/g;s/6/▇/g;s/7/█/g;")
        .to_owned()
}

/// Returns the amount of bars that should be present.
fn get_bars(config: &'static Config) -> i32 {
    let bars = config.read_config_raw()[HYBRID_ROOT_JSON]["cava_bars"]
        .as_i32()
        .unwrap_or(5);
    bars.clamp(2, 16)
}

/// Returns the desired framerate to use for Cava updates.
fn get_framerate(config: &'static Config) -> i32 {
    let framerate = config.read_config_raw()[HYBRID_ROOT_JSON]["cava_framerate"]
        .as_i32()
        .unwrap_or(60);
    framerate.clamp(60, 360)
}

/// Builds the temporary Cava configuration and then returns the path to it,
pub fn get_temp_config(config: &'static Config) -> String {
    let path = CAVA_TMP_CONFIG.to_owned();
    // 0.2.7: Support for dynamically configuring the temporary config to an extent.
    let bars = get_bars(config);
    let framerate = get_framerate(config);
    let mut conf = include_str!("../../resources/cava_tmp.conf");
    let formatted = conf
        .replace("[framerate]", &framerate.to_string())
        .replace("[bars]", &bars.to_string());

    conf = &formatted;
    write(&path, conf).expect(ERR_WRITE_TMP_CONF);
    path
}

/// Updates the `BARS` value with Cava.
/// Only call this once as it's a loop.
pub fn update_bars(config: &'static Config) {
    task::spawn(async move {
        let mut bars;
        let sed = get_sed(config);
        let path = get_temp_config(config);
        let mut child = Command::new(PROC_TARGET)
            .args(["-c", &format!("cava -p {path} | sed -u '{sed}'")])
            .stdout(Stdio::piped())
            .kill_on_drop(true)
            .spawn()
            .expect(ERR_START_CAVA);

        let out = child.stdout.take().expect(ERR_TAKE_STDOUT);

        // Drop to free the resources as we don't need to access them anymore.
        drop(path);
        drop(sed);
        let mut reader = BufReader::new(out).lines();
        loop {
            bars = reader
                .next_line()
                .await
                .unwrap_or_else(|_| on_cava_crashed())
                .unwrap_or_else(|| on_cava_crashed());

            if let Ok(mut w_bars) = BARS.write() {
                *w_bars = bars;
            }
        }
    });
}

/// Called when Cava has crashed.
fn on_cava_crashed() -> ! {
    *HAS_CAVA_CRASHED.write().unwrap() = true;
    BARS.write().unwrap().clear();
    panic!("{}", WARN_CAVA_NO_LINES)
}
