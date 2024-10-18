use crate::utils;
use binn_rs::{List, Map, Object, Value};

#[test]
fn primitives() {
    let bytes = utils::read_encoded_file("obj/primitives");
    let value = Value::deserialize(bytes.as_slice()).unwrap();
    let list: Object = value.try_into().unwrap();

    let expected = vec![
        ("v_null", Value::Null),
        ("v_true", Value::True),
        ("v_false", Value::False),
        ("n_u8", Value::UInt8(62)),
        ("n_i8", Value::Int8(61)),
        ("n_u16", Value::UInt16(6262)),
        ("n_i16", Value::Int16(6161)),
        ("n_u32", Value::UInt32(62626262)),
        ("n_i32", Value::Int32(61616161)),
        ("n_f32", Value::Float(0.6262)),
        ("n_u64", Value::UInt64(6262626262626262)),
        ("n_i64", Value::Int64(6161616161616161)),
        ("n_f64", Value::Double(0.6161)),
        ("s_text", Value::Text("Text")),
        ("s_datetime", Value::DateTime("DateTime")),
        ("s_date", Value::Date("Date")),
        ("s_time", Value::Time("Time")),
        ("s_decimal", Value::DecimalStr("Decimal")),
        ("b_blob", Value::Blob(&[0x62, 0x61, 0x62, 0x61])),
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
    let bytes = utils::read_encoded_file("obj/user_types");
    let value = Value::deserialize(bytes.as_slice()).unwrap();
    let list: Object = value.try_into().unwrap();

    let expected = vec![
        ("empty1", Value::Empty(5.into())),
        ("empty2", Value::Empty(20.into())),
        ("byte1", Value::Byte(6.into(), 62)),
        ("byte2", Value::Byte(40.into(), 61)),
        ("word1", Value::Word(7.into(), 6262)),
        ("word2", Value::Word(80.into(), 6161)),
        ("dword1", Value::DWord(8.into(), 62626262)),
        ("dword2", Value::DWord(160.into(), 61616161)),
        ("qword1", Value::QWord(9.into(), 6262626262626262)),
        (
            "qword2",
            Value::QWord(320.try_into().unwrap(), 6161616161616161),
        ),
        ("text1", Value::UserText(10.into(), "Text")),
        ("text2", Value::UserText(645.try_into().unwrap(), "Date")),
        (
            "blob1",
            Value::UserBlob(15.into(), &[0x62, 0x61, 0x62, 0x61]),
        ),
        (
            "blob2",
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
    let bytes = utils::read_encoded_file("obj/containers");
    let value = Value::deserialize(bytes.as_slice()).unwrap();
    let obj: Object = value.try_into().unwrap();

    assert_eq!(obj.count(), 3);

    let mut iter = obj.iter();
    let child_expected = vec![
        (-257978445, "v_null", Value::Null),
        (257978445, "n_u8", Value::UInt8(62)),
        (42, "n_i8", Value::Int8(61)),
    ];

    let (key, child_list) = iter.next().unwrap();
    assert_eq!(key, "list");
    let child_list: List = child_list.try_into().unwrap();
    assert_eq!(child_list.count(), child_expected.len());

    for (ref actual, (_, _, expected)) in child_list.iter().zip(child_expected.iter()) {
        assert_eq!(actual, expected);
    }

    let (key, child_map) = iter.next().unwrap();
    assert_eq!(key, "map");
    let child_map: Map = child_map.try_into().unwrap();
    assert_eq!(child_map.count(), child_expected.len());

    for ((ref actual_key, ref actual_val), (expected_key, _, expected_val)) in
        child_map.iter().zip(child_expected.iter())
    {
        assert_eq!(actual_key, expected_key);
        assert_eq!(actual_val, expected_val);
    }

    let (key, child_obj) = iter.next().unwrap();
    assert_eq!(key, "obj");
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
