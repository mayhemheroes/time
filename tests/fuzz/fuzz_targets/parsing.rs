#![no_main]

use libfuzzer_sys::fuzz_target;
use time::format_description::well_known::{Iso8601, Rfc2822, Rfc3339};
use time::OffsetDateTime;

fuzz_target!(|data: &str| {
    let _ = OffsetDateTime::parse(data, &Rfc2822);
    let _ = OffsetDateTime::parse(data, &Rfc3339);
    let _ = OffsetDateTime::parse(data, &Iso8601::DEFAULT);
    let _ = OffsetDateTime::parse(data, &Iso8601::PARSING);
});
