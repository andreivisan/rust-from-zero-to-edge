use std::time::Duration;

use time::PrimitiveDateTime as DateTime;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    let duration_gigaseconds = Duration::from_secs(1000000000);
    let after_date = match start.checked_add(duration_gigaseconds) {
        Some(value) => { value }
        None => { panic!("Date overflow"); }

    };
    after_date
}
