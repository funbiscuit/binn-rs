use crate::utils;
use binn_rs::{List, Map, Object, Value};

#[test]
fn primitives() {
    let expected = utils::read_encoded_file("map/primitives");

    let mut buf = vec![0; 512];
    let mut map = Map::empty_mut(buf.as_mut_slice()).unwrap();

    map.add_value(1, Value::Null).unwrap();
    map.add_value(10, Value::True).unwrap();
    map.add_value(20, Value::False).unwrap();

    map.add_value(30, Value::UInt8(62)).unwrap();
    map.add_value(40, Value::Int8(61)).unwrap();

    map.add_value(50, Value::UInt16(6262)).unwrap();
    map.add_value(100, Value::Int16(6161)).unwrap();

    map.add_value(150, Value::UInt32(62626262)).unwrap();
    map.add_value(200, Value::Int32(61616161)).unwrap();
    map.add_value(500, Value::Float(0.6262)).unwrap();

    map.add_value(1_000, Value::UInt64(6262626262626262))
        .unwrap();
    map.add_value(10_000, Value::Int64(6161616161616161))
        .unwrap();
    map.add_value(100_000, Value::Double(0.6161)).unwrap();

    map.add_value(200_000, Value::Text("Text")).unwrap();
    map.add_value(5_000_000, Value::DateTime("DateTime"))
        .unwrap();
    map.add_value(10_000_000, Value::Date("Date")).unwrap();
    map.add_value(50_000_000, Value::Time("Time")).unwrap();
    map.add_value(1_000_000_000, Value::DecimalStr("Decimal"))
        .unwrap();

    map.add_value(2_000_000_000, Value::Blob(&[0x62, 0x61, 0x62, 0x61]))
        .unwrap();

    assert_eq!(expected, map.as_bytes());
}

#[test]
fn user_types() {
    let expected = utils::read_encoded_file("map/user_types");

    let mut buf = vec![0; 512];
    let mut obj = Map::empty_mut(buf.as_mut_slice()).unwrap();

    obj.add_value(10, Value::Empty(5.into())).unwrap();
    obj.add_value(-10, Value::Empty(20.into())).unwrap();

    obj.add_value(20, Value::Byte(6.into(), 62)).unwrap();
    obj.add_value(-20, Value::Byte(40.into(), 61)).unwrap();

    obj.add_value(287, Value::Word(7.into(), 6262)).unwrap();
    obj.add_value(-287, Value::Word(80.into(), 6161)).unwrap();

    obj.add_value(1234, Value::DWord(8.into(), 62626262))
        .unwrap();
    obj.add_value(-1234, Value::DWord(160.into(), 61616161))
        .unwrap();

    obj.add_value(5654, Value::QWord(9.into(), 6262626262626262))
        .unwrap();
    obj.add_value(
        -5654,
        Value::QWord(320.try_into().unwrap(), 6161616161616161),
    )
    .unwrap();

    obj.add_value(2756423, Value::UserText(10.into(), "Text"))
        .unwrap();
    obj.add_value(-2756423, Value::UserText(645.try_into().unwrap(), "Date"))
        .unwrap();

    obj.add_value(
        2147483647,
        Value::UserBlob(15.into(), &[0x62, 0x61, 0x62, 0x61]),
    )
    .unwrap();
    obj.add_value(
        -2147483648,
        Value::UserBlob(4095.try_into().unwrap(), &[0x61, 0x62, 0x61, 0x62]),
    )
    .unwrap();

    assert_eq!(expected, obj.as_bytes());
}

#[test]
fn containers() {
    let expected = utils::read_encoded_file("map/containers");

    let mut buf = vec![0; 512];
    let mut map = Map::empty_mut(buf.as_mut_slice()).unwrap();

    let mut child: List = map
        .add_value(10, List::empty())
        .unwrap()
        .try_into()
        .unwrap();

    child.add_value(Value::Null).unwrap();
    child.add_value(Value::UInt8(62)).unwrap();
    child.add_value(Value::Int8(61)).unwrap();

    let mut child: Map = map.add_value(20, Map::empty()).unwrap().try_into().unwrap();

    child.add_value(-257978445, Value::Null).unwrap();
    child.add_value(257978445, Value::UInt8(62)).unwrap();
    child.add_value(42, Value::Int8(61)).unwrap();

    let mut child_obj: Object = map
        .add_value(30, Object::empty())
        .unwrap()
        .try_into()
        .unwrap();

    child_obj.add_value("v_null", Value::Null).unwrap();
    child_obj.add_value("n_u8", Value::UInt8(62)).unwrap();
    child_obj.add_value("n_i8", Value::Int8(61)).unwrap();

    assert_eq!(expected, map.as_bytes());
}
