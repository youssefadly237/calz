# Calz &emsp; [![Build Status]][actions] [![Latest Version]][crates.io]

[Build Status]: https://img.shields.io/github/actions/workflow/status/youssefadly237/calz/ci.yml?branch=main
[actions]: https://github.com/youssefadly237/calz/actions?query=branch%3Amain
[Latest Version]: https://img.shields.io/crates/v/calz.svg
[crates.io]: https://crates.io/crates/calz

**Status: Work in Progress**  
This library is **not production-ready**. APIs and calculations may change, and
bugs are expected. Please **do not use in production** until
a stable release is announced.

Calz (calendars(z)) is a Rust library for multi-calendar conversions.
Inspired by the book _Calendrical Calculations_ by Dershowitz & Reingold.
It aims to be a scientific reference for calendar arithmetic and conversions.

## Supported Calendars

- Coptic
- Gregorian
- Islamic

See [docs/INTRO.md](./docs/INTRO.md) for a full explanation of calendar, and formulas.

## Installation

```bash
cargo add calz

```

## Disclaimer

This crate is for educational and research purposes.
This crate was developed by translating known calendar algorithms into Rust.
Despite careful testing, there may be mistakes in calculations or edge cases.
**DO NOT** use for mission-critical or scientific purposes without verifying results.

## Contributing

Contributions, bug reports, and feature requests are welcome!  
Feel free to open an [issue](https://github.com/youssefadly237/calz/issues)
or submit a pull request.
All contributions must comply with the GPLv3 license.

## License

`calz` is licensed under the **GNU General Public License v3 (GPLv3)**.

- You may use, modify, and distribute this software commercially.
- Any derivative work must also be released under **GPLv3**.
- See [LICENSE](./LICENSE) for the full text.
