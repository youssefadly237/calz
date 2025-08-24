use calz::calendars::coptic::{Coptic, Date as CopticDate, Variant as CopticVariant};
use calz::calendars::gregorian::{Date as GDate, Gregorian, Variant as GVar};
use calz::calendars::islamic::{Date as IslamicDate, Islamic, Variant as IslamicVariant};
use calz::core::{context::Context, traits::convert};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ctx = Context::default();

    // Gregorian input
    let g_in = GDate {
        year: 2025,
        month: 8,
        day: 23,
    };

    // Gregorian → Coptic
    let coptic_out: CopticDate = convert::<Gregorian, Coptic>(
        &g_in,
        &GVar::Proleptic,
        &CopticVariant::Proleptic,
        Some(&ctx),
    )?;
    println!(
        "Gregorian {:?} → Coptic {:?} {}",
        g_in,
        coptic_out,
        Coptic::MONTH_NAMES[(coptic_out.month - 1) as usize]
    );
    // Gregorian → Coptic
    let islamic_out: IslamicDate =
        convert::<Gregorian, Islamic>(&g_in, &GVar::Proleptic, &IslamicVariant::Civil, Some(&ctx))?;
    println!(
        "Gregorian {:?} → Islamic {:?} {}",
        g_in,
        islamic_out,
        Islamic::MONTH_NAMES[(islamic_out.month - 1) as usize]
    );

    // Reverse: Coptic → Gregorian
    let g_back: GDate = convert::<Coptic, Gregorian>(
        &coptic_out,
        &CopticVariant::Proleptic,
        &GVar::Proleptic,
        Some(&ctx),
    )?;
    println!(
        "Coptic {:?} {} → Gregorian {:?}",
        coptic_out,
        Coptic::MONTH_NAMES[(coptic_out.month - 1) as usize],
        g_back
    );

    Ok(())
}
