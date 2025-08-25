use super::epoch::EpochDay;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A generic context struct providing extra information to calendar computations.
///
/// This struct allows calendars to use optional parameters for astronomical calculations,
/// historical adjustments, or calendar-specific rules. It can be passed to `to_epoch_day`
/// and `from_epoch_day` methods in the `Calendar` trait.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Context {
    /// Geographic longitude in degrees.
    /// Used for astronomical calendars to calculate local sunrise, moon phases, etc.
    pub longitude_deg: Option<f64>,

    /// Geographic latitude in degrees.
    /// Used for astronomical calendars to calculate local sunrise, moon phases, etc.
    pub latitude_deg: Option<f64>,

    /// Delta T in seconds (difference between Terrestrial Time and Universal Time).
    /// Relevant for high-precision astronomical calculations.
    pub delta_t_seconds: Option<f64>,

    /// Optional timezone offset from UTC in hours.
    /// Helps convert astronomical events to local civil dates.
    pub timezone_offset_hours: Option<f64>,

    /// Optional custom epoch override.
    /// Useful for calendars that have a non-standard or adjustable epoch (start date).
    pub custom_epoch: Option<EpochDay>,

    /// Calendar-specific options stored as key-value pairs.
    /// Can be used to specify rules like leap months, era variants, or algorithmic choices.
    #[serde(default)]
    pub options: HashMap<String, String>, // Use empty map by default

    /// Historical cutoff dates for calendars that changed over time.
    /// Example: Julian→Gregorian transition, stored as `{ "GregorianStart": EpochDay(...) }`.
    #[serde(default)]
    pub cutoffs: HashMap<String, EpochDay>, // Use empty map by default
}

impl Context {
    /// Example validation method
    pub fn validate(&self) -> Result<(), String> {
        if let Some(lon) = self.longitude_deg
            && !(-180.0..=180.0).contains(&lon)
        {
            return Err("Longitude out of range".into());
        }
        if let Some(lat) = self.latitude_deg
            && !(-90.0..=90.0).contains(&lat)
        {
            return Err("Latitude out of range".into());
        }
        Ok(())
    }
}
