use rog_types::{aura_modes::AuraModes, gfx_vendors::GfxVendors};
use serde_derive::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::config::{Config, Profile};

/// for parsing old v2.1.2 config
#[derive(Deserialize)]
pub(crate) struct ConfigV212 {
    gfx_managed: bool,
    bat_charge_limit: u8,
    active_profile: String,
    toggle_profiles: Vec<String>,
    power_profiles: BTreeMap<String, Profile>,
    power_profile: u8,
    kbd_led_brightness: u8,
    kbd_backlight_mode: u8,
    kbd_backlight_modes: Vec<AuraModes>,
}

impl ConfigV212 {
    pub(crate) fn into_current(self) -> Config {
        Config {
            gfx_mode: GfxVendors::Hybrid,
            gfx_managed: self.gfx_managed,
            active_profile: self.active_profile,
            toggle_profiles: self.toggle_profiles,
            curr_fan_mode: self.power_profile,
            bat_charge_limit: self.bat_charge_limit,
            kbd_led_brightness: self.kbd_led_brightness,
            kbd_backlight_mode: self.kbd_backlight_mode,
            kbd_backlight_modes: self.kbd_backlight_modes,
            power_profiles: self.power_profiles,
        }
    }
}

/// for parsing old v2.2.2 config
#[derive(Deserialize)]
pub(crate) struct ConfigV222 {
    gfx_managed: bool,
    bat_charge_limit: u8,
    active_profile: String,
    toggle_profiles: Vec<String>,
    power_profiles: BTreeMap<String, Profile>,
    power_profile: u8,
    kbd_led_brightness: u8,
    kbd_backlight_mode: u8,
    kbd_backlight_modes: Vec<AuraModes>,
}

impl ConfigV222 {
    pub(crate) fn into_current(self) -> Config {
        Config {
            gfx_mode: GfxVendors::Hybrid,
            gfx_managed: self.gfx_managed,
            active_profile: self.active_profile,
            toggle_profiles: self.toggle_profiles,
            curr_fan_mode: self.power_profile,
            bat_charge_limit: self.bat_charge_limit,
            kbd_led_brightness: self.kbd_led_brightness,
            kbd_backlight_mode: self.kbd_backlight_mode,
            kbd_backlight_modes: self.kbd_backlight_modes,
            power_profiles: self.power_profiles,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub(crate) struct ConfigV301 {
    pub gfx_managed: bool,
    pub gfx_nv_mode_is_dedicated: bool,
    pub active_profile: String,
    pub toggle_profiles: Vec<String>,
    // TODO: remove power_profile
    #[serde(skip)]
    pub curr_fan_mode: u8,
    pub bat_charge_limit: u8,
    pub kbd_led_brightness: u8,
    pub kbd_backlight_mode: u8,
    pub kbd_backlight_modes: Vec<AuraModes>,
    pub power_profiles: BTreeMap<String, Profile>,
}

impl ConfigV301 {
    pub(crate) fn into_current(self) -> Config {
        Config {
            gfx_mode: GfxVendors::Hybrid,
            gfx_managed: self.gfx_managed,
            active_profile: self.active_profile,
            toggle_profiles: self.toggle_profiles,
            curr_fan_mode: self.curr_fan_mode,
            bat_charge_limit: self.bat_charge_limit,
            kbd_led_brightness: self.kbd_led_brightness,
            kbd_backlight_mode: self.kbd_backlight_mode,
            kbd_backlight_modes: self.kbd_backlight_modes,
            power_profiles: self.power_profiles,
        }
    }
}