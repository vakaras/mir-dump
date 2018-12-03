// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use lazy_static::lazy_static;
use std::sync::RwLock;
use std::env;
use config::{Config, Environment, File};

lazy_static! {
    // Is this RwLock<..> necessary?
    static ref SETTINGS: RwLock<Config> = RwLock::new({
        let mut settings = Config::default();

        // 1. Default values
        settings.set_default("LOG_DIR", "./log/").unwrap();
        settings.set_default::<Option<String>>("DUMP_MIR_PROC", None).unwrap();
        settings.set_default("DUMP_MIR_INFO", true).unwrap();
        settings.set_default("DUMP_SHOW_TEMP_VARIABLES", true).unwrap();
        settings.set_default("DUMP_SHOW_STATEMENT_INDICES", true).unwrap();
        settings.set_default("DUMP_DEBUG_INFO", false).unwrap();
        settings.set_default("TEST", false).unwrap();
        settings.set_default("FULL_COMPILATION", true).unwrap();

        // 2. Override with the optional TOML file "mir_dump.toml" (if there is any)
        settings.merge(
            File::with_name("mir_dump.toml").required(false)
        ).unwrap();

        // 3. Override with an optional TOML file specified by the `MIR_DUMP_CONFIG` env variable
        settings.merge(
            File::with_name(&env::var("MIR_DUMP_CONFIG").unwrap_or("".to_string())).required(false)
        ).unwrap();

        // 4. Override with env variables (`MIR_DUMP_CONFIG_DUMP_MIR_PROC`, ...)
        settings.merge(
            Environment::with_prefix("MIR_DUMP").ignore_empty(true).separator(",")
        ).unwrap();

        settings
	});
}

/// Generate a dump of the settings
pub fn dump() -> String {
    format!("{:?}", SETTINGS.read().unwrap())
}

/// Should we dump borrowck info?
pub fn dump_mir_info() -> bool {
    SETTINGS.read().unwrap().get::<bool>("DUMP_MIR_INFO").unwrap()
}

/// Should the mir dump show temporary variables?
pub fn dump_show_temp_variables() -> bool {
    SETTINGS.read().unwrap().get::<bool>("DUMP_SHOW_TEMP_VARIABLES").unwrap()
}

/// Should the mir dump show temporary variables?
pub fn dump_show_statement_indices() -> bool {
    SETTINGS.read().unwrap().get::<bool>("DUMP_SHOW_STATEMENT_INDICES").unwrap()
}

/// The function of which MIR info should be dumped.
pub fn dump_mir_proc() -> Option<String> {
    SETTINGS.read().unwrap().get::<Option<String>>("DUMP_MIR_PROC").unwrap()
}

/// In which folder should we sore log/dumps?
pub fn log_dir() -> String {
    SETTINGS.read().unwrap().get::<String>("LOG_DIR").unwrap()
}

/// Should we dump debug files?
pub fn dump_debug_info() -> bool {
    SETTINGS.read().unwrap().get::<bool>("DUMP_DEBUG_INFO").unwrap()
}

/// Are we running under test?
pub fn test() -> bool {
    SETTINGS.read().unwrap().get::<bool>("TEST").unwrap()
}

/// Are we running under test?
pub fn full_compilation() -> bool {
    SETTINGS.read().unwrap().get::<bool>("FULL_COMPILATION").unwrap()
}
