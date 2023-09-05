use std::collections::BTreeMap;

use serde::Deserialize;

use crate::ser::Value;

#[derive(Deserialize, Debug)]
struct A {
    b: B,
    foo: u8,
}
#[derive(Deserialize, Debug)]
struct B {
    c: C,
    foo_str: String,
}
#[derive(Deserialize, Debug)]
struct C(i8, u64);

#[test]
fn test_dep() {
    let c_value = Value::TupleStruct(vec![Value::Int8(22), Value::UInt64(33)]);
    let b_value = Value::Struct({
        let mut map = BTreeMap::new();
        map.insert(Value::StructKey("c".to_string()), c_value);
        map.insert(
            Value::StructKey("foo_str".to_string()),
            Value::String("hello".to_string()),
        );
        map
    });
    let a_value = Value::Struct({
        let mut map = BTreeMap::new();
        map.insert(Value::StructKey("b".to_string()), b_value);
        map.insert(Value::StructKey("foo".to_string()), Value::UInt8(11));
        map
    });

    let res = A::deserialize(a_value).unwrap();

    assert_eq!(
        format!("{res:?}"),
        r#"A { b: B { c: C(22, 33), foo_str: "hello" }, foo: 11 }"#
    );
}
