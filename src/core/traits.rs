use super::{context::Context, epoch::EpochDay, error::CalError};

/// A trait for calendar systems that support conversion to and from a common epoch day.
///
/// Implementors define how to convert their calendar's date representation to an [`EpochDay`]
/// and back, optionally using a [`Variant`] and a [`Context`] for calendar-specific rules,
/// astronomical parameters, or historical adjustments.
///
/// # Associated Types
/// - `Date`: The calendar's native date type.
/// - `Variant`: A type representing calendar variants (e.g., era, algorithm, leap rules).
pub trait Calendar {
    type Date;
    type Variant;

    /// Converts a calendar date to an [`EpochDay`] (days since the reference epoch).
    ///
    /// # Arguments
    /// - `date`: The calendar's native date type.
    /// - `var`: The calendar variant (e.g., era, leap rule).
    /// - `ctx`: Optional context for astronomical or historical parameters.
    ///
    /// # Errors
    /// Returns [`CalError`] if the date is invalid or conversion fails.
    fn to_epoch_day(
        date: &Self::Date,
        var: &Self::Variant,
        ctx: Option<&Context>,
    ) -> Result<EpochDay, CalError>;

    /// Converts an [`EpochDay`] to a calendar date.
    ///
    /// # Arguments
    /// - `ed`: The epoch day to convert.
    /// - `var`: The calendar variant (e.g., era, leap rule).
    /// - `ctx`: Optional context for astronomical or historical parameters.
    ///
    /// # Errors
    /// Returns [`CalError`] if the epoch day is out of range or conversion fails.
    fn from_epoch_day(
        ed: EpochDay,
        var: &Self::Variant,
        ctx: Option<&Context>,
    ) -> Result<Self::Date, CalError>;
}

/// Converts a date from one calendar system to another via the epoch hub.
///
/// This function first converts the source calendar's date to an [`EpochDay`],
/// then converts that epoch day to the target calendar's date type.
///
/// # Arguments
/// - `a_date`: The source calendar's date.
/// - `a_var`: The source calendar's variant.
/// - `b_var`: The target calendar's variant.
/// - `ctx`: Optional context for astronomical or historical parameters.
///
/// # Errors
/// Returns [`CalError`] if either conversion fails.
pub fn convert<A: Calendar, B: Calendar>(
    a_date: &A::Date,
    a_var: &A::Variant,
    b_var: &B::Variant,
    ctx: Option<&Context>,
) -> Result<B::Date, CalError> {
    let ed = A::to_epoch_day(a_date, a_var, ctx)?;
    B::from_epoch_day(ed, b_var, ctx)
}
