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
    Civil, // tabular (arithmetical) calendar
}

pub struct Islamic;

impl Islamic {
    // Islamic epoch: 622-07-16 (Julian) = RD 227015
    const EPOCH: i64 = 227013;

    #[inline]
    fn is_leap(y: i64) -> bool {
        // Leap years in a 30-year cycle occur 11 times
        ((11 * y + 14) % 30) < 11
    }

    fn ymd_to_rd(y: i64, m: i64, d: i64) -> Result<i64, CalError> {
        if !(1..=12).contains(&m) {
            return Err(CalError::InvalidDate);
        }
        if d < 1 {
            return Err(CalError::InvalidDate);
        }

        // Month lengths: alternate 30 / 29, last month 29 or 30 in leap year
        let mdays = match m {
            1 | 3 | 5 | 7 | 9 | 11 => 30,
            12 => {
                if Self::is_leap(y) {
                    30
                } else {
                    29
                }
            }
            _ => 29,
        };
        if d > mdays {
            return Err(CalError::InvalidDate);
        }

        // Days before this year
        let y1 = y - 1;
        let days_before_year = 354 * y1 + div_floor(3 + 11 * y1, 30);

        // Day-of-year
        let mut doy = 0;
        for mm in 1..m {
            doy += match mm {
                1 | 3 | 5 | 7 | 9 | 11 => 30,
                12 => {
                    if Self::is_leap(y) {
                        30
                    } else {
                        29
                    }
                }
                _ => 29,
            };
        }
        doy += d - 1;

        Ok(Self::EPOCH + days_before_year + doy)
    }

    fn rd_to_ymd(rd: i64) -> (i64, i64, i64) {
        let days = rd - Self::EPOCH;

        // Approximate year
        let year = div_floor(30 * days + 10646, 10631);
        let start_of_year = 354 * (year - 1) + div_floor(3 + 11 * (year - 1), 30);
        let day_of_year = days - start_of_year;

        // Find month
        let mut m = 1;
        let mut days_passed = 0;
        loop {
            let mdays = match m {
                1 | 3 | 5 | 7 | 9 | 11 => 30,
                12 => {
                    if Self::is_leap(year) {
                        30
                    } else {
                        29
                    }
                }
                _ => 29,
            };
            if day_of_year < days_passed + mdays {
                let d = day_of_year - days_passed + 1;
                return (year, m, d);
            }
            days_passed += mdays;
            m += 1;
        }
    }

    // Month names (Hijri)
    pub const MONTH_NAMES: [&'static str; 12] = [
        "Muharram",
        "Safar",
        "Rabi‘ al-awwal",
        "Rabi‘ al-thani",
        "Jumada al-ula",
        "Jumada al-akhira",
        "Rajab",
        "Sha‘ban",
        "Ramadan",
        "Shawwal",
        "Dhu al-Qi‘dah",
        "Dhu al-Hijjah",
    ];
    pub const MONTH_NAMES_ARABIC: [&'static str; 12] = [
        "مُحَرَّم",
        "صَفَر",
        "رَبيع الأوّل",
        "رَبيع الآخر",
        "جُمادى الأولى",
        "جُمادى الآخرة",
        "رَجَب",
        "شَعبان",
        "رَمَضان",
        "شوّال",
        "ذو القعدة",
        "ذو الحجة",
    ];
}

impl Calendar for Islamic {
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
