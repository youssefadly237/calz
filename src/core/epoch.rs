use serde::{Deserialize, Serialize};

/// Represents a count of days relative to a fixed reference point (an epoch).
///
/// In this library, [`EpochDay`] is defined as the number of days since
/// the **proleptic Gregorian calendar** date `0001-01-01` (midnight).
/// This convention is consistent with the *Rata Die* (RD) system used in
/// calendrical calculations.
///
/// # Semantics
/// - `EpochDay(0)` corresponds to `0001-01-01` (Gregorian).
/// - Positive values move forward in time (e.g., `EpochDay(1)` = 0001-01-02).
/// - Negative values move backward (e.g., `EpochDay(-1)` = 0000-12-31).
///
/// # Use cases
/// [`EpochDay`] is a *calendar-agnostic* intermediate type.  
/// Conversion algorithms for various calendars (Gregorian, Julian, Islamic, etc.)
/// usually normalize dates into `EpochDay` first, then convert back.
///
/// # Example
/// ```
/// use calz::core::epoch::{EpochDay, RD_EPOCH};
///
/// // The reference epoch
/// assert_eq!(RD_EPOCH, EpochDay(0));
///
/// // Days are counted forward and backward
/// assert_eq!(EpochDay(1).0, 1);  // 0001-01-02
/// assert_eq!(EpochDay(-1).0, -1); // 0000-12-31
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct EpochDay(pub i64);

/// The Rata Die (RD) epoch: proleptic Gregorian 0001-01-01 (midnight).
///
/// This serves as the universal reference point for all `EpochDay` values.
pub const RD_EPOCH: EpochDay = EpochDay(0);
