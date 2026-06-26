mod common;

use example_app::generated;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_get_current_time() {
    let pool = common::get_pool().await;
    let time = generated::admin::get_current_time(pool).await.unwrap();
    assert!(time.is_some());
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_get_version() {
    let pool = common::get_pool().await;
    let version = generated::admin::get_version(pool).await.unwrap();
    assert!(version.is_some());
    assert!(version.unwrap().contains("PostgreSQL"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_insert_and_get_all_types() {
    use jiff::civil;
    use jiff_sqlx::ToSqlx;
    use rust_decimal::Decimal;
    use sqlx::postgres::types::{PgInterval, PgRange, PgTimeTz};
    use std::str::FromStr;
    use uuid::Uuid;

    let pool = common::get_pool().await;

    let bool_col = true;
    let char_col = "A".to_string();
    let int2_col: i16 = 32767;
    let int4_col: i32 = 2147483647;
    let int8_col: i64 = 9223372036854775807;
    let float4_col: f32 = 3.14159;
    let float8_col: f64 = 2.718281828459045;
    let numeric_col = Decimal::from_str("12345.67").unwrap();
    let name_col = "test_name".to_string();
    let text_col = "This is a test text".to_string();
    let varchar_col = "varchar test".to_string();
    let bpchar_col = "bpchar    ".to_string();
    let bytea_col = vec![0xDE, 0xAD, 0xBE, 0xEF];

    let mut bit_col = bit_vec::BitVec::from_elem(8, false);
    bit_col.set(0, true);
    bit_col.set(2, true);
    bit_col.set(4, true);
    bit_col.set(6, true);

    let mut varbit_col = bit_vec::BitVec::from_elem(16, false);
    varbit_col.set(0, true);
    varbit_col.set(2, true);
    varbit_col.set(4, true);
    varbit_col.set(6, true);
    varbit_col.set(8, true);
    varbit_col.set(10, true);
    varbit_col.set(12, true);
    varbit_col.set(14, true);

    let date_col = civil::date(2025, 11, 20).to_sqlx();
    let time_col = civil::time(14, 30, 0, 0).to_sqlx();
    let timestamp_col = civil::date(2025, 11, 20).at(14, 30, 0, 0).to_sqlx();
    let timestamptz_col = "2025-11-20T14:30:00Z"
        .parse::<jiff::Timestamp>()
        .unwrap()
        .to_sqlx();
    let interval_col = PgInterval {
        months: 0,
        days: 1,
        microseconds: (2 * 3600 + 30 * 60) * 1_000_000,
    };
    let timetz_col = PgTimeTz {
        time: time::Time::from_hms(14, 30, 0).unwrap(),
        offset: time::UtcOffset::UTC,
    };

    let int4_range_col =
        PgRange::from((std::ops::Bound::Included(1), std::ops::Bound::Excluded(10)));
    let int8_range_col = PgRange::from((
        std::ops::Bound::Included(100i64),
        std::ops::Bound::Included(200i64),
    ));
    let num_range_col = PgRange::from((
        std::ops::Bound::Included(Decimal::from_str("0.5").unwrap()),
        std::ops::Bound::Included(Decimal::from_str("99.9").unwrap()),
    ));
    let ts_range_col = PgRange::from((
        std::ops::Bound::Included(time::PrimitiveDateTime::new(
            time::Date::from_calendar_date(2025, time::Month::January, 1).unwrap(),
            time::Time::MIDNIGHT,
        )),
        std::ops::Bound::Included(time::PrimitiveDateTime::new(
            time::Date::from_calendar_date(2025, time::Month::December, 31).unwrap(),
            time::Time::from_hms(23, 59, 59).unwrap(),
        )),
    ));
    let tstz_range_col = PgRange::from((
        std::ops::Bound::Included(
            time::PrimitiveDateTime::new(
                time::Date::from_calendar_date(2025, time::Month::January, 1).unwrap(),
                time::Time::MIDNIGHT,
            )
            .assume_offset(time::UtcOffset::UTC),
        ),
        std::ops::Bound::Included(
            time::PrimitiveDateTime::new(
                time::Date::from_calendar_date(2025, time::Month::December, 31).unwrap(),
                time::Time::from_hms(23, 59, 59).unwrap(),
            )
            .assume_offset(time::UtcOffset::UTC),
        ),
    ));
    let date_range_col = PgRange::from((
        std::ops::Bound::Included(
            time::Date::from_calendar_date(2025, time::Month::January, 1).unwrap(),
        ),
        std::ops::Bound::Included(
            time::Date::from_calendar_date(2025, time::Month::December, 31).unwrap(),
        ),
    ));

    let inet_col: std::net::IpAddr = "192.168.1.1".parse().unwrap();
    let cidr_col: std::net::IpAddr = "192.168.1.0".parse().unwrap();
    let macaddr_col = mac_address::MacAddress::new([0x08, 0x00, 0x2b, 0x01, 0x02, 0x03]);

    let json_col = serde_json::json!({"key": "value", "number": 42});
    let jsonb_col = serde_json::json!({"name": "test", "tags": ["tag1", "tag2"]});
    let uuid_col = Uuid::parse_str("a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11").unwrap();

    let bool_array_col = vec![true, false, true];
    let int4_array_col = vec![1, 2, 3, 4, 5];
    let int8_array_col = vec![100i64, 200i64, 300i64];
    let text_array_col = vec!["one".to_string(), "two".to_string(), "three".to_string()];
    let float8_array_col = vec![1.1, 2.2, 3.3];

    let int4_range_array_col = vec![
        PgRange::from((std::ops::Bound::Included(1), std::ops::Bound::Excluded(5))),
        PgRange::from((std::ops::Bound::Included(10), std::ops::Bound::Excluded(20))),
    ];
    let date_range_array_col = vec![
        PgRange::from((
            std::ops::Bound::Included(
                time::Date::from_calendar_date(2025, time::Month::January, 1).unwrap(),
            ),
            std::ops::Bound::Included(
                time::Date::from_calendar_date(2025, time::Month::January, 31).unwrap(),
            ),
        )),
        PgRange::from((
            std::ops::Bound::Included(
                time::Date::from_calendar_date(2025, time::Month::June, 1).unwrap(),
            ),
            std::ops::Bound::Included(
                time::Date::from_calendar_date(2025, time::Month::June, 30).unwrap(),
            ),
        )),
    ];

    let id = generated::admin::insert_all_types_test(
        pool,
        bool_col,
        char_col,
        int2_col,
        int4_col,
        int8_col,
        float4_col,
        float8_col,
        numeric_col,
        name_col,
        text_col,
        varchar_col,
        bpchar_col,
        bytea_col,
        bit_col,
        varbit_col,
        date_col,
        time_col,
        timestamp_col,
        timestamptz_col,
        interval_col,
        timetz_col,
        int4_range_col,
        int8_range_col,
        num_range_col,
        ts_range_col,
        tstz_range_col,
        date_range_col,
        inet_col,
        cidr_col,
        macaddr_col,
        json_col,
        jsonb_col,
        uuid_col,
        bool_array_col,
        int4_array_col,
        int8_array_col,
        text_array_col,
        float8_array_col,
        int4_range_array_col,
        date_range_array_col,
    )
    .await
    .unwrap();

    let retrieved = generated::admin::get_all_types_test(pool, id)
        .await
        .unwrap();
    assert_eq!(retrieved.bool_col, Some(true));
    assert_eq!(retrieved.int4_col, Some(2147483647));
    assert_eq!(retrieved.int8_col, Some(9223372036854775807i64));
    assert_eq!(
        retrieved.uuid_col,
        Some(Uuid::parse_str("a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11").unwrap())
    );
}
