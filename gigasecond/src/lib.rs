use time::{PrimitiveDateTime as DateTime, Duration};

pub fn after(start: DateTime) -> DateTime {
    let giga_second = (10 as i64).pow(9);
    start + Duration::seconds(giga_second)
}
