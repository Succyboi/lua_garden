use std::env;
use super::module_content::ConstModuleContent;

pub const INTERNAL_INCLUDES: [(&str, &str); 7] = [
    (include_str!("../lua/_internal/includes/runtime.lua"), "runtime.lua"),
    (include_str!("../lua/_internal/includes/math_extensions.lua"), "math_extensions.lua"),
    (include_str!("../lua/_internal/includes/pitch.lua"), "pitch.lua"),
    (include_str!("../lua/_internal/includes/buffer.lua"), "buffer.lua"),
    (include_str!("../lua/_internal/includes/parameter.lua"), "parameter.lua"),
    (include_str!("../lua/_internal/includes/gen.lua"), "gen.lua"),
    (include_str!("../lua/_internal/includes/filters.lua"), "filters.lua")
];

pub const INIT_HEADER: &str = include_str!("../lua/_internal/headers/init_header.lua");
pub const RESET_HEADER: &str = include_str!("../lua/_internal/headers/reset_header.lua");
pub const TRIGGER_HEADER: &str = include_str!("../lua/_internal/headers/trigger_header.lua");
pub const RUN_HEADER: &str = include_str!("../lua/_internal/headers/run_header.lua");
pub const INIT_FOOTER: &str = include_str!("../lua/_internal/footers/init_footer.lua");
pub const RESET_FOOTER: &str = include_str!("../lua/_internal/footers/reset_footer.lua");
pub const TRIGGER_FOOTER: &str = include_str!("../lua/_internal/footers/trigger_footer.lua");
pub const RUN_FOOTER: &str = include_str!("../lua/_internal/footers/run_footer.lua");

pub const INIT_PATH: &str = "init.lua";
pub const RESET_PATH: &str = "reset.lua";
pub const TRIGGER_PATH: &str = "trigger.lua";
pub const RUN_PATH: &str = "run.lua";
pub const INTERFACE_PATH: &str = "interface.lua";

pub const DEFAULT_INIT_CONTENT: &str = include_str!("../lua/_default/init.lua");
pub const DEFAULT_RESET_CONTENT: &str = include_str!("../lua/_default/reset.lua");
pub const DEFAULT_TRIGGER_CONTENT: &str = include_str!("../lua/_default/trigger.lua");
pub const DEFAULT_RUN_CONTENT: &str = include_str!("../lua/_default/run.lua");
pub const DEFAULT_INTERFACE_CONTENT: &str = include_str!("../lua/_default/interface.lua");

pub const MODULE_DEFAULT: ConstModuleContent = ConstModuleContent::new(
    include_str!("../lua/_default/init.lua"),
    include_str!("../lua/_default/reset.lua"),
    include_str!("../lua/_default/trigger.lua"),
    include_str!("../lua/_default/run.lua"),
    include_str!("../lua/_default/interface.lua"));

pub const MODULE_EXAMPLES: [(ConstModuleContent, &str); 4] = [
    (ConstModuleContent::new(
        include_str!("../lua/examples/0_noise/init.lua"),
        DEFAULT_RESET_CONTENT,
        DEFAULT_TRIGGER_CONTENT,
        include_str!("../lua/examples/0_noise/run.lua"),
        DEFAULT_INTERFACE_CONTENT), // TODO
        "Noise"),

    (ConstModuleContent::new(
        include_str!("../lua/examples/1_bitcrusher/init.lua"),
        include_str!("../lua/examples/1_bitcrusher/reset.lua"),
        DEFAULT_TRIGGER_CONTENT,
        include_str!("../lua/examples/1_bitcrusher/run.lua"),
        DEFAULT_INTERFACE_CONTENT), // TODO
        "Bitcrusher"),

    (ConstModuleContent::new(
        include_str!("../lua/examples/2_dj_filter/init.lua"),
        include_str!("../lua/examples/2_dj_filter/reset.lua"),
        DEFAULT_TRIGGER_CONTENT,
        include_str!("../lua/examples/2_dj_filter/run.lua"),
        DEFAULT_INTERFACE_CONTENT), // TODO
        "DJ Filter"),

    (ConstModuleContent::new(
        include_str!("../lua/examples/3_waveshaper/init.lua"),
        DEFAULT_RESET_CONTENT,
        DEFAULT_TRIGGER_CONTENT,
        include_str!("../lua/examples/3_waveshaper/run.lua"),
        DEFAULT_INTERFACE_CONTENT), // TODO
        "Waveshaper"),
];

pub fn internal_includes() -> String {
    let mut includes = String::new();

    for include in INTERNAL_INCLUDES {
        includes.push_str(&format!(
            "\n\
            -- ==== --\n\
            -- INCLUDE {}\n\
            -- ↓↓↓↓ --\n", include.1));
        includes.push_str(include.0);
        includes.push_str(&format!(
            "\n\
            -- ↑↑↑↑ --\n\
            -- INCLUDE {}\n\
            -- ==== --\n", include.1));
    }

    return  includes;
}

pub fn default_workspaces_path () -> String {
    let mut workdir_path = match env::current_dir() {
        Ok(path) => path,
        Err(_e) => return String::new(),
    };

    workdir_path.push("workspaces");
    match workdir_path.to_str() {
        Some(p) => return String::from(p),
        None => return String::new()
    }
}