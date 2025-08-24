use thiserror::Error;

#[derive(Debug, Error)]
pub enum CalError {
    #[error("invalid date")]
    InvalidDate,
    #[error("day out of range for month")]
    DayOutOfRange,
    #[error("month out of range")]
    MonthOutOfRange,
    #[error("out of supported range")]
    OutOfRange,
    #[error("variant needs specific context")]
    NeedsContext,
    #[error("invalid context provided")]
    InvalidContext,
    #[error("missing astronomical parameter delta_t")]
    MissingDeltaT,
    #[error("table-backed variant not enabled")]
    TableMissing,
    #[error("unknown era")]
    UnknownEra,
    #[error("conversion failed")]
    ConversionFailed,
    #[error("arithmetic overflow")]
    Overflow,
    #[error("arithmetic underflow")]
    Underflow,
}
