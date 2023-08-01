use binn_rs::{List, Map, Object, Value};

#[test]
fn list() {
    let mut buf = vec![0; 512];
    let mut list = List::empty_mut(buf.as_mut_slice()).unwrap();

    list.add_value(Value::Null).unwrap();
    list.add_value(Value::UInt16(6262)).unwrap();

    assert_eq!(list.get(0).unwrap(), Value::Null);
    assert_eq!(list.get(1).unwrap(), Value::UInt16(6262));
    assert_eq!(list.get(2), None);
}

#[test]
fn map() {
    let mut buf = vec![0; 512];
    let mut map = Map::empty_mut(buf.as_mut_slice()).unwrap();

    map.add_value(1, Value::Null).unwrap();
    map.add_value(50, Value::UInt16(6262)).unwrap();

    println!("{:?}", map.as_bytes());

    assert_eq!(map.get(1).unwrap(), Value::Null);
    assert_eq!(map.get(50).unwrap(), Value::UInt16(6262));
    assert_eq!(map.get(10), None);
}

#[test]
fn obj() {
    let mut buf = vec![0; 512];
    let mut obj = Object::empty_mut(buf.as_mut_slice()).unwrap();

    obj.add_value("v_null", Value::Null).unwrap();
    obj.add_value("n_u16", Value::UInt16(6262)).unwrap();

    assert_eq!(obj.get("v_null").unwrap(), Value::Null);
    assert_eq!(obj.get("n_u16").unwrap(), Value::UInt16(6262));
    assert_eq!(obj.get("something"), None);
}
