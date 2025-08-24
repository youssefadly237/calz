use crate::core::{context::Context, epoch::EpochDay, error::CalError, traits::Calendar};
use crate::util::math::div_floor;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Date {
    pub year: i32,
    pub month: u8,
    pub day: u8,
}

#[derive(Debug, Clone, Copy)]
pub enum Variant {
    Proleptic,
}

pub struct Coptic;

impl Coptic {
    const EPOCH: i64 = 103604;

    #[inline]
    fn is_leap(y: i64) -> bool {
        y % 4 == 3 // Coptic leap rule: year mod 4 == 3
    }

    fn ymd_to_rd(y: i64, m: i64, d: i64) -> Result<i64, CalError> {
        if !(1..=13).contains(&m) {
            return Err(CalError::InvalidDate);
        }
        if d < 1 {
            return Err(CalError::InvalidDate);
        }

        // Days in month
        let mdays = if m == 13 {
            if Self::is_leap(y) { 6 } else { 5 }
        } else {
            30
        };
        if d > mdays {
            return Err(CalError::InvalidDate);
        }

        let days_before_year = 365 * (y - 1) + div_floor(y - 1, 4);
        let doy = 30 * (m - 1) + (d - 1);
        Ok(Self::EPOCH + days_before_year + doy)
    }

    fn rd_to_ymd(rd: i64) -> (i64, i64, i64) {
        let days = rd - Self::EPOCH;
        let year = div_floor(days, 1461) * 4 + div_floor(days % 1461, 365) + 1;
        let day_of_year = days - (365 * (year - 1) + div_floor(year - 1, 4));

        let (month, day) = if day_of_year < 30 * 12 {
            (day_of_year / 30 + 1, day_of_year % 30 + 1)
        } else {
            (13, day_of_year - 30 * 12 + 1)
        };
        (year, month, day)
    }

    // https://www.copticchurch.net/calendar
    pub const MONTH_NAMES: [&'static str; 13] = [
        "Thoout",
        "Paopi",
        "Hathor",
        "Koiak",
        "Tobi",
        "Meshir",
        "Paremhat",
        "Paremoude",
        "Pashons",
        "Paoni",
        "Epip",
        "Mesori",
        "Pi Kogi Enavot",
    ];
    pub const MONTH_NAMES_MODERN: [&'static str; 13] = [
        "Tout",
        "Baba",
        "Hator",
        "Kiahk",
        "Toba",
        "Amshir",
        "Baramhat",
        "Baramouda",
        "Baramouda",
        "Paona",
        "Epep",
        "Mesra",
        "Nasie",
    ];
    pub const MONTH_NAMES_COPTIC: [&'static str; 13] = [
        "Ⲑⲱⲟⲩⲧ",
        "Ⲡⲁⲟⲡⲓ",
        "Ⲁⲑⲟⲣ",
        "Ⲭⲟⲓⲁⲕ",
        "Ⲧⲱⲃⲓ",
        "Ⲙⲉϣⲓⲣ",
        "Ⲡⲁⲣⲉⲙϩⲁⲧ",
        "Ⲫⲁⲣⲙⲟⲑⲓ",
        "Ⲡⲁϣⲁⲛⲥ",
        "Ⲡⲁ̀ⲱⲛ",
        "̀Ⲉⲡⲏⲡ",
        "Ⲙⲉⲥⲱⲣⲏ",
        "Ⲡⲓⲕⲟⲩϫⲓ ̀ⲛ̀ⲁⲃⲟⲧ",
    ];
    pub const MONTH_NAMES_ARABIC: [&'static str; 13] = [
        "توت",
        "بابه",
        "هاتور",
        "كيهك",
        "طوبه",
        "أمشير",
        "برمهات",
        "برموده",
        "بشنس",
        "بؤونه",
        "أبيب",
        "مسرى",
        "النسيء",
    ];
}

impl Calendar for Coptic {
    type Date = Date;
    type Variant = Variant;

    fn to_epoch_day(
        date: &Self::Date,
        _var: &Self::Variant,
        _ctx: Option<&Context>,
    ) -> Result<EpochDay, CalError> {
        let rd = Self::ymd_to_rd(date.year as i64, date.month as i64, date.day as i64)?;
        Ok(EpochDay(rd))
    }

    fn from_epoch_day(
        ed: EpochDay,
        _var: &Self::Variant,
        _ctx: Option<&Context>,
    ) -> Result<Self::Date, CalError> {
        let (y, m, d) = Self::rd_to_ymd(ed.0);
        Ok(Date {
            year: y as i32,
            month: m as u8,
            day: d as u8,
        })
    }
}
