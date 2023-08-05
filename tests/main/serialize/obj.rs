use crate::utils;
use binn_rs::{List, Map, Object, Value};

#[test]
fn primitives() {
    let expected = utils::read_encoded_file("obj/primitives");

    let mut buf = vec![0; 512];
    let mut obj = Object::empty_mut(buf.as_mut_slice()).unwrap();

    obj.add_value("v_null", Value::Null).unwrap();
    obj.add_value("v_true", true).unwrap();
    obj.add_value("v_false", false).unwrap();

    obj.add_value("n_u8", 62u8).unwrap();
    obj.add_value("n_i8", 61i8).unwrap();

    obj.add_value("n_u16", 6262u16).unwrap();
    obj.add_value("n_i16", 6161i16).unwrap();

    obj.add_value("n_u32", 62626262u32).unwrap();
    obj.add_value("n_i32", 61616161i32).unwrap();
    obj.add_value("n_f32", 0.6262f32).unwrap();

    obj.add_value("n_u64", 6262626262626262u64).unwrap();
    obj.add_value("n_i64", 6161616161616161i64).unwrap();
    obj.add_value("n_f64", 0.6161f64).unwrap();

    obj.add_value("s_text", "Text").unwrap();
    obj.add_value("s_datetime", Value::DateTime("DateTime"))
        .unwrap();
    obj.add_value("s_date", Value::Date("Date")).unwrap();
    obj.add_value("s_time", Value::Time("Time")).unwrap();
    obj.add_value("s_decimal", Value::DecimalStr("Decimal"))
        .unwrap();

    obj.add_value("b_blob", [0x62, 0x61, 0x62, 0x61].as_slice())
        .unwrap();

    assert_eq!(expected, obj.as_bytes());
}

#[test]
fn user_types() {
    let expected = utils::read_encoded_file("obj/user_types");

    let mut buf = vec![0; 512];
    let mut obj = Object::empty_mut(buf.as_mut_slice()).unwrap();

    obj.add_value("empty1", Value::Empty(5.into())).unwrap();
    obj.add_value("empty2", Value::Empty(20.into())).unwrap();

    obj.add_value("byte1", Value::Byte(6.into(), 62)).unwrap();
    obj.add_value("byte2", Value::Byte(40.into(), 61)).unwrap();

    obj.add_value("word1", Value::Word(7.into(), 6262)).unwrap();
    obj.add_value("word2", Value::Word(80.into(), 6161))
        .unwrap();

    obj.add_value("dword1", Value::DWord(8.into(), 62626262))
        .unwrap();
    obj.add_value("dword2", Value::DWord(160.into(), 61616161))
        .unwrap();

    obj.add_value("qword1", Value::QWord(9.into(), 6262626262626262))
        .unwrap();
    obj.add_value(
        "qword2",
        Value::QWord(320.try_into().unwrap(), 6161616161616161),
    )
    .unwrap();

    obj.add_value("text1", Value::UserText(10.into(), "Text"))
        .unwrap();
    obj.add_value("text2", Value::UserText(645.try_into().unwrap(), "Date"))
        .unwrap();

    obj.add_value(
        "blob1",
        Value::UserBlob(15.into(), &[0x62, 0x61, 0x62, 0x61]),
    )
    .unwrap();
    obj.add_value(
        "blob2",
        Value::UserBlob(4095.try_into().unwrap(), &[0x61, 0x62, 0x61, 0x62]),
    )
    .unwrap();

    assert_eq!(expected, obj.as_bytes());
}

#[test]
fn containers() {
    let expected = utils::read_encoded_file("obj/containers");

    let mut buf = vec![0; 512];
    let mut obj = Object::empty_mut(buf.as_mut_slice()).unwrap();

    let mut child: List = obj
        .add_value("list", List::empty())
        .unwrap()
        .try_into()
        .unwrap();

    child.add_value(Value::Null).unwrap();
    child.add_value(62u8).unwrap();
    child.add_value(61i8).unwrap();

    let mut child: Map = obj
        .add_value("map", Map::empty())
        .unwrap()
        .try_into()
        .unwrap();

    child.add_value(-257978445, Value::Null).unwrap();
    child.add_value(257978445, 62u8).unwrap();
    child.add_value(42, 61i8).unwrap();

    let mut child_obj: Object = obj
        .add_value("obj", Object::empty())
        .unwrap()
        .try_into()
        .unwrap();

    child_obj.add_value("v_null", Value::Null).unwrap();
    child_obj.add_value("n_u8", 62u8).unwrap();
    child_obj.add_value("n_i8", 61i8).unwrap();

    assert_eq!(expected, obj.as_bytes());
}
