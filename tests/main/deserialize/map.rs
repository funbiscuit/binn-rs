use crate::utils;
use binn_rs::{List, Map, Object, Value};

#[test]
fn primitives() {
    let bytes = utils::read_encoded_file("map/primitives");
    let value: Value = bytes.as_slice().try_into().unwrap();
    let list: Map = value.try_into().unwrap();

    let expected = vec![
        (1, Value::Null),
        (10, Value::True),
        (20, Value::False),
        (30, Value::UInt8(62)),
        (40, Value::Int8(61)),
        (50, Value::UInt16(6262)),
        (100, Value::Int16(6161)),
        (150, Value::UInt32(62626262)),
        (200, Value::Int32(61616161)),
        (500, Value::Float(0.6262)),
        (1_000, Value::UInt64(6262626262626262)),
        (10_000, Value::Int64(6161616161616161)),
        (100_000, Value::Double(0.6161)),
        (200_000, Value::Text("Text")),
        (5_000_000, Value::DateTime("DateTime")),
        (10_000_000, Value::Date("Date")),
        (50_000_000, Value::Time("Time")),
        (1_000_000_000, Value::DecimalStr("Decimal")),
        (2_000_000_000, Value::Blob(&[0x62, 0x61, 0x62, 0x61])),
    ];

    assert_eq!(expected.len(), list.count());

    for ((ref actual_key, ref actual_val), (expected_key, expected_val)) in
        list.iter().zip(expected.iter())
    {
        assert_eq!(actual_key, expected_key);
        assert_eq!(actual_val, expected_val);
    }
}

#[test]
fn user_types() {
    let bytes = utils::read_encoded_file("map/user_types");
    let value: Value = bytes.as_slice().try_into().unwrap();
    let list: Map = value.try_into().unwrap();

    let expected = vec![
        (10, Value::Empty(5.into())),
        (-10, Value::Empty(20.into())),
        (20, Value::Byte(6.into(), 62)),
        (-20, Value::Byte(40.into(), 61)),
        (287, Value::Word(7.into(), 6262)),
        (-287, Value::Word(80.into(), 6161)),
        (1234, Value::DWord(8.into(), 62626262)),
        (-1234, Value::DWord(160.into(), 61616161)),
        (5654, Value::QWord(9.into(), 6262626262626262)),
        (
            -5654,
            Value::QWord(320.try_into().unwrap(), 6161616161616161),
        ),
        (2756423, Value::UserText(10.into(), "Text")),
        (-2756423, Value::UserText(645.try_into().unwrap(), "Date")),
        (
            2147483647,
            Value::UserBlob(15.into(), &[0x62, 0x61, 0x62, 0x61]),
        ),
        (
            -2147483648,
            Value::UserBlob(4095.try_into().unwrap(), &[0x61, 0x62, 0x61, 0x62]),
        ),
    ];

    assert_eq!(expected.len(), list.count());

    for ((ref actual_key, ref actual_val), (expected_key, expected_val)) in
        list.iter().zip(expected.iter())
    {
        assert_eq!(actual_key, expected_key);
        assert_eq!(actual_val, expected_val);
    }
}

#[test]
fn containers() {
    let bytes = utils::read_encoded_file("map/containers");
    let value: Value = bytes.as_slice().try_into().unwrap();
    let map: Map = value.try_into().unwrap();

    assert_eq!(map.count(), 3);

    let mut iter = map.iter();
    let child_expected = vec![
        (-257978445, "v_null", Value::Null),
        (257978445, "n_u8", Value::UInt8(62)),
        (42, "n_i8", Value::Int8(61)),
    ];

    let (key, child_list) = iter.next().unwrap();
    assert_eq!(key, 10);
    let child_list: List = child_list.try_into().unwrap();
    assert_eq!(child_list.count(), child_expected.len());

    for (ref actual, (_, _, expected)) in child_list.iter().zip(child_expected.iter()) {
        assert_eq!(actual, expected);
    }

    let (key, child_map) = iter.next().unwrap();
    assert_eq!(key, 20);
    let child_map: Map = child_map.try_into().unwrap();
    assert_eq!(child_map.count(), child_expected.len());

    for ((ref actual_key, ref actual_val), (expected_key, _, expected_val)) in
        child_map.iter().zip(child_expected.iter())
    {
        assert_eq!(actual_key, expected_key);
        assert_eq!(actual_val, expected_val);
    }

    let (key, child_obj) = iter.next().unwrap();
    assert_eq!(key, 30);
    let child_obj: Object = child_obj.try_into().unwrap();
    assert_eq!(child_obj.count(), child_expected.len());

    for ((ref actual_key, ref actual_val), (_, expected_key, expected_val)) in
        child_obj.iter().zip(child_expected.iter())
    {
        assert_eq!(actual_key, expected_key);
        assert_eq!(actual_val, expected_val);
    }

    assert_eq!(iter.next(), None);
}
