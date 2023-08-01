use crate::utils;
use binn_rs::{List, Map, Object, Value};

#[test]
fn primitives() {
    let bytes = utils::read_encoded_file("list/primitives");
    let value: Value = bytes.as_slice().try_into().unwrap();
    let list: List = value.try_into().unwrap();

    let expected = vec![
        Value::Null,
        Value::True,
        Value::False,
        Value::UInt8(62),
        Value::Int8(61),
        Value::UInt16(6262),
        Value::Int16(6161),
        Value::UInt32(62626262),
        Value::Int32(61616161),
        Value::Float(0.6262),
        Value::UInt64(6262626262626262),
        Value::Int64(6161616161616161),
        Value::Double(0.6161),
        Value::Text("Text"),
        Value::DateTime("DateTime"),
        Value::Date("Date"),
        Value::Time("Time"),
        Value::DecimalStr("Decimal"),
        Value::Blob(&[0x62, 0x61, 0x62, 0x61]),
    ];

    assert_eq!(expected.len(), list.count());

    for (ref actual, expected) in list.iter().zip(expected.iter()) {
        assert_eq!(actual, expected);
    }
}

#[test]
fn user_types() {
    let bytes = utils::read_encoded_file("list/user_types");
    let value: Value = bytes.as_slice().try_into().unwrap();
    let list: List = value.try_into().unwrap();

    let expected = vec![
        Value::Empty(5.into()),
        Value::Empty(20.into()),
        Value::Byte(6.into(), 62),
        Value::Byte(40.into(), 61),
        Value::Word(7.into(), 6262),
        Value::Word(80.into(), 6161),
        Value::DWord(8.into(), 62626262),
        Value::DWord(160.into(), 61616161),
        Value::QWord(9.into(), 6262626262626262),
        Value::QWord(320.try_into().unwrap(), 6161616161616161),
        Value::UserText(10.into(), "Text"),
        Value::UserText(645.try_into().unwrap(), "Date"),
        Value::UserBlob(15.into(), &[0x62, 0x61, 0x62, 0x61]),
        Value::UserBlob(4095.try_into().unwrap(), &[0x61, 0x62, 0x61, 0x62]),
    ];

    assert_eq!(expected.len(), list.count());

    for (ref actual, expected) in list.iter().zip(expected.iter()) {
        assert_eq!(actual, expected);
    }
}

#[test]
fn containers() {
    let bytes = utils::read_encoded_file("list/containers");
    let value: Value = bytes.as_slice().try_into().unwrap();
    let list: List = value.try_into().unwrap();

    assert_eq!(list.count(), 3);

    let mut iter = list.iter();
    let child_expected = vec![
        (-257978445, "v_null", Value::Null),
        (257978445, "n_u8", Value::UInt8(62)),
        (42, "n_i8", Value::Int8(61)),
    ];

    let child_list: List = iter.next().unwrap().try_into().unwrap();
    assert_eq!(child_list.count(), child_expected.len());

    for (ref actual, (_, _, expected)) in child_list.iter().zip(child_expected.iter()) {
        assert_eq!(actual, expected);
    }

    let child_map: Map = iter.next().unwrap().try_into().unwrap();
    assert_eq!(child_map.count(), child_expected.len());

    for ((ref actual_key, ref actual_val), (expected_key, _, expected_val)) in
        child_map.iter().zip(child_expected.iter())
    {
        assert_eq!(actual_key, expected_key);
        assert_eq!(actual_val, expected_val);
    }

    let child_obj: Object = iter.next().unwrap().try_into().unwrap();
    assert_eq!(child_obj.count(), child_expected.len());

    for ((ref actual_key, ref actual_val), (_, expected_key, expected_val)) in
        child_obj.iter().zip(child_expected.iter())
    {
        assert_eq!(actual_key, expected_key);
        assert_eq!(actual_val, expected_val);
    }

    assert_eq!(iter.next(), None);
}
