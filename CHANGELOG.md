# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

# 0.10.0 (2022-12-27)
## Fixed
- Fix panic due to incorrect ANSI escape handling ([#137])
- Fix display of empty tables ([#127])

## Changed
- Remove the unsafe code in `Table::as_ref` ([#146])
- Switch `atty` to `is-terminal` ([#151])
- Minimal Supported Rust Version bumped to 1.56

## Thanks
- @alexanderkjall and @5225225 fuzzer work and fixing panics
- @david0u0 fixing ([#145]) Undefined behavior (UB) on `Table::as_ref`

[#127]: https://github.com/phsym/prettytable-rs/pull/127
[#137]: https://github.com/phsym/prettytable-rs/pull/137
[#145]: https://github.com/phsym/prettytable-rs/issues/145
[#146]: https://github.com/phsym/prettytable-rs/pull/146
[#151]: https://github.com/phsym/prettytable-rs/pull/151
