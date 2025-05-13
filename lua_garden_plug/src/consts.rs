pub const ICON: &str = "\u{EBAE}";
pub const NAME: &str = env!("CARGO_PKG_NAME");
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
#[cfg(debug_assertions)] pub const BUILD_TYPE: &str = "debug";
#[cfg(not(debug_assertions))] pub const BUILD_TYPE: &str = "release";
pub const BUILD_ID: &str = env!("BUILD_ID");

pub const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
pub const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
pub const DISCLAIMER: &str = "THIS SOFTWARE IS EXPERIMENTAL.\n\
    This software has not been publicly released.\n\
    It is not ready for production and you are using it at your own risk.";
pub const LICENSE_NAME: &str = "ANTI-CAPITALIST SOFTWARE LICENSE v 1.4";
pub const LICENSE_CONTENTS: &str = include_str!("../LICENSE");
pub const CREDITS: &str = include_str!("../credits.txt");

pub const MOTD: &str = "Have fun. ♥";

pub const WINDOW_SIZE_WIDTH: u32 = 640;
pub const WINDOW_SIZE_HEIGHT: u32 = 512;

pub const DARKMODE_DEFAULT: bool = true;