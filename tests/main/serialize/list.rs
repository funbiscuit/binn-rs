use crate::utils;
use binn_rs::{List, Map, Object, Value};

#[test]
fn primitives() {
    let expected = utils::read_encoded_file("list/primitives");

    let mut buf = vec![0; 512];
    let mut list = List::empty_mut(buf.as_mut_slice()).unwrap();

    list.add_value(Value::Null).unwrap();
    list.add_value(Value::True).unwrap();
    list.add_value(Value::False).unwrap();

    list.add_value(Value::UInt8(62)).unwrap();
    list.add_value(Value::Int8(61)).unwrap();

    list.add_value(Value::UInt16(6262)).unwrap();
    list.add_value(Value::Int16(6161)).unwrap();

    list.add_value(Value::UInt32(62626262)).unwrap();
    list.add_value(Value::Int32(61616161)).unwrap();
    list.add_value(Value::Float(0.6262)).unwrap();

    list.add_value(Value::UInt64(6262626262626262)).unwrap();
    list.add_value(Value::Int64(6161616161616161)).unwrap();
    list.add_value(Value::Double(0.6161)).unwrap();

    list.add_value(Value::Text("Text")).unwrap();
    list.add_value(Value::DateTime("DateTime")).unwrap();
    list.add_value(Value::Date("Date")).unwrap();
    list.add_value(Value::Time("Time")).unwrap();
    list.add_value(Value::DecimalStr("Decimal")).unwrap();

    list.add_value(Value::Blob(&[0x62, 0x61, 0x62, 0x61]))
        .unwrap();

    assert_eq!(expected, list.as_bytes());
}

#[test]
fn user_types() {
    let expected = utils::read_encoded_file("list/user_types");

    let mut buf = vec![0; 512];
    let mut list = List::empty_mut(buf.as_mut_slice()).unwrap();

    list.add_value(Value::Empty(5.into())).unwrap();
    list.add_value(Value::Empty(20.into())).unwrap();

    list.add_value(Value::Byte(6.into(), 62)).unwrap();
    list.add_value(Value::Byte(40.into(), 61)).unwrap();

    list.add_value(Value::Word(7.into(), 6262)).unwrap();
    list.add_value(Value::Word(80.into(), 6161)).unwrap();

    list.add_value(Value::DWord(8.into(), 62626262)).unwrap();
    list.add_value(Value::DWord(160.into(), 61616161)).unwrap();

    list.add_value(Value::QWord(9.into(), 6262626262626262))
        .unwrap();
    list.add_value(Value::QWord(320.try_into().unwrap(), 6161616161616161))
        .unwrap();

    list.add_value(Value::UserText(10.into(), "Text")).unwrap();
    list.add_value(Value::UserText(645.try_into().unwrap(), "Date"))
        .unwrap();

    list.add_value(Value::UserBlob(15.into(), &[0x62, 0x61, 0x62, 0x61]))
        .unwrap();
    list.add_value(Value::UserBlob(
        4095.try_into().unwrap(),
        &[0x61, 0x62, 0x61, 0x62],
    ))
    .unwrap();

    assert_eq!(expected, list.as_bytes());
}

#[test]
fn containers() {
    let expected = utils::read_encoded_file("list/containers");

    let mut buf = vec![0; 512];
    let mut list = List::empty_mut(buf.as_mut_slice()).unwrap();

    let mut child: List = list.add_value(List::empty()).unwrap().try_into().unwrap();

    child.add_value(Value::Null).unwrap();
    child.add_value(Value::UInt8(62)).unwrap();
    child.add_value(Value::Int8(61)).unwrap();

    let mut child: Map = list.add_value(Map::empty()).unwrap().try_into().unwrap();

    child.add_value(-257978445, Value::Null).unwrap();
    child.add_value(257978445, Value::UInt8(62)).unwrap();
    child.add_value(42, Value::Int8(61)).unwrap();

    let mut child_obj: Object = list.add_value(Object::empty()).unwrap().try_into().unwrap();

    child_obj.add_value("v_null", Value::Null).unwrap();
    child_obj.add_value("n_u8", Value::UInt8(62)).unwrap();
    child_obj.add_value("n_i8", Value::Int8(61)).unwrap();

    assert_eq!(expected, list.as_bytes());
}
