use chrono::NaiveDateTime;
use serde::Serializer;

pub fn format_date_time<S>(dt: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let formatted = dt.format("%Y-%m-%d %H:%M:%S").to_string();
    serializer.serialize_str(&formatted)
}

pub fn format_option_date_time<S>(
    dt: &Option<NaiveDateTime>,
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

#[cfg(test)]
mod tests {
    use chrono::NaiveDateTime;
    use serde::Serialize;

    use super::*;

    #[derive(Serialize)]
    struct WithTime {
        #[serde(serialize_with = "format_date_time")]
        pub dt: NaiveDateTime,
    }

    #[derive(Serialize)]
    struct WithOptTime {
        #[serde(serialize_with = "format_option_date_time")]
        pub dt: Option<NaiveDateTime>,
    }

    #[test]
    fn format_date_time_serializes_correctly() {
        let dt = NaiveDateTime::parse_from_str("2026-06-17 10:30:45", "%Y-%m-%d %H:%M:%S").unwrap();
        let json = serde_json::to_string(&WithTime { dt }).unwrap();
        assert_eq!(json, r#"{"dt":"2026-06-17 10:30:45"}"#);
    }

    #[test]
    fn format_option_date_time_with_value() {
        let dt = NaiveDateTime::parse_from_str("2026-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let json = serde_json::to_string(&WithOptTime { dt: Some(dt) }).unwrap();
        assert_eq!(json, r#"{"dt":"2026-01-01 00:00:00"}"#);
    }

    #[test]
    fn format_option_date_time_none() {
        let json = serde_json::to_string(&WithOptTime { dt: None }).unwrap();
        assert_eq!(json, r#"{"dt":null}"#);
    }
}
