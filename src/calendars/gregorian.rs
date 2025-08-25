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
    Cutover,
}

pub struct Gregorian;

impl Gregorian {
    #[inline]
    fn is_leap(y: i64) -> bool {
        (y % 4 == 0) && (y % 100 != 0 || y % 400 == 0)
    }

    /// Days before month in a non-leap year.
    #[inline]
    fn doy_prefix(month: i64, leap: bool) -> i64 {
        // 1-based month; 0 for Jan base
        const CUM: [i64; 13] = [0, 0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];
        CUM[month as usize] + if leap && month > 2 { 1 } else { 0 }
    }

    /// Convert Y-M-D to days since 0001-01-01 (RD epoch) using integer math.
    fn ymd_to_rd(y: i64, m: i64, d: i64) -> Result<i64, CalError> {
        if !(1..=12).contains(&m) {
            return Err(CalError::InvalidDate);
        }
        if !(1..=31).contains(&d) {
            return Err(CalError::InvalidDate);
        }

        let leap = Self::is_leap(y);
        let mdays = match m {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                if leap {
                    29
                } else {
                    28
                }
            }
            _ => unreachable!(),
        };
        if d > mdays {
            return Err(CalError::InvalidDate);
        }

        // Count days in previous full years (Gregorian arithmetic)
        let y1 = y - 1;
        let days_before_year =
            365 * y1 + div_floor(y1, 4) - div_floor(y1, 100) + div_floor(y1, 400);

        // Day-of-year offset
        let doy = Self::doy_prefix(m, leap) + d - 1;

        // RD day 0 == 0001-01-01
        Ok(days_before_year + doy)
    }

    /// Inverse: RD → Y-M-D (Hinnant-style integer inverse; no loops).
    fn rd_to_ymd(rd: i64) -> (i64, i64, i64) {
        // Convert RD where 0001-01-01 = 0 to civil y-m-d.
        // Decompose by 400/100/4/1-year cycles.
        let mut z = rd;

        let quad400 = div_floor(z, 146097); // days in 400y
        z -= quad400 * 146097;

        // Handle the last day of a 400y cycle carefully
        let mut quad100 = z / 36524;
        if quad100 == 4 {
            quad100 = 3;
        }
        z -= quad100 * 36524;

        let quad4 = z / 1461;
        z -= quad4 * 1461;

        let mut year1 = z / 365;
        if year1 == 4 {
            year1 = 3;
        }
        z -= year1 * 365;

        let year = 400 * quad400 + 100 * quad100 + 4 * quad4 + year1 + 1;

        // z is day-of-year (0-based)
        let leap = Self::is_leap(year);
        // month by binary search over cumulative days (small table)
        const CUM_N: [i64; 13] = [0, 0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];
        const CUM_L: [i64; 13] = [0, 0, 31, 60, 91, 121, 152, 182, 213, 244, 274, 305, 335];
        let cum = if leap { &CUM_L } else { &CUM_N };

        // find largest m with cum[m] <= z
        let mut m = 1;
        while m < 12 && cum[(m + 1) as usize] <= z {
            m += 1;
        }
        let day = z - cum[m as usize] + 1;

        (year, m as i64, day)
    }
}

impl Calendar for Gregorian {
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
