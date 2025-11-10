use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct SudoConfig<'a> {
    pub format: &'a str,
    pub cached_symbol: &'a str,
    pub uncached_symbol: &'a str,
    pub allow_windows: bool,
    pub disabled: bool,
}

impl Default for SudoConfig<'_> {
    fn default() -> Self {
        Self {
            format: "[as $symbol](bold blue)",
            cached_symbol: "🧙 ",
            uncached_symbol: "👤 ",
            allow_windows: false,
            disabled: true,
        }
    }
}
