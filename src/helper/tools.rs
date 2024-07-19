use sea_orm::prelude::DateTimeLocal;
use serde::Serializer;

pub fn format_date_time<S>(dt: &DateTimeLocal, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let formatted = dt.format("%Y-%m-%d %H:%M:%S").to_string();
    serializer.serialize_str(&formatted)
}

pub fn format_option_date_time<S>(
    dt: &Option<DateTimeLocal>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match dt {
        Some(dt) => format_date_time(dt, serializer),
        None => serializer.serialize_none(),
    }
}
