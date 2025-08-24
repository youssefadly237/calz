use super::{context::Context, epoch::EpochDay, error::CalError};

pub trait Calendar {
    type Date;
    type Variant;

    fn to_epoch_day(
        date: &Self::Date,
        var: &Self::Variant,
        ctx: Option<&Context>,
    ) -> Result<EpochDay, CalError>;

    fn from_epoch_day(
        ed: EpochDay,
        var: &Self::Variant,
        ctx: Option<&Context>,
    ) -> Result<Self::Date, CalError>;
}

/// Generic any→any conversion via the hub.
pub fn convert<A: Calendar, B: Calendar>(
    a_date: &A::Date,
    a_var: &A::Variant,
    b_var: &B::Variant,
    ctx: Option<&Context>,
) -> Result<B::Date, CalError> {
    let ed = A::to_epoch_day(a_date, a_var, ctx)?;
    B::from_epoch_day(ed, b_var, ctx)
}
