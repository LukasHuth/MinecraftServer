use std::collections::HashMap;

use crate::{reader::NbtReader, version::Java, traits::NbtRead, NbtValue};

#[test]
fn test_hello_world() {
    let file = include_bytes!("../test_data/hello_world.nbt");
    let file = file.to_vec();
    let reader = NbtReader::new(file);
    let value = Java::from_reader(reader);
    assert!(value.is_ok());
    let value = value.unwrap();
    let expected_values = vec![("name".to_string(), NbtValue::String("Bananrama".to_string()))];
    match value {
        NbtValue::Compound(name, data) => {
            assert!(name.is_some());
            assert_eq!(name, Some("hello world".to_string()));
            for value in data {
                assert!(expected_values.contains(&value));
            }
        },
        _ => assert!(false),
    }
}
#[test]
fn test_big_test() {
    let file = include_bytes!("../test_data/bigtest.nbt");
    let file = file.to_vec();
    let reader = NbtReader::from_compressed_data(file);
    let value = Java::from_reader(reader);
    assert!(value.is_ok());
    let value = value.unwrap();
    let expected_values: HashMap<String, NbtValue> = vec![
        ("stringTest".to_string(), NbtValue::String("HELLO WORLD THIS IS A TEST STRING ÅÄÖ!".to_string())),
        ("intTest".to_string(), NbtValue::Int(2147483647)),
        ("byteTest".to_string(), NbtValue::Byte(127)),
        ("listTest (long)".to_string(), NbtValue::List(vec![
            NbtValue::Long(11),
            NbtValue::Long(12),
            NbtValue::Long(13),
            NbtValue::Long(14),
            NbtValue::Long(15),
        ])),
        ("doubleTest".to_string(), NbtValue::Double(0.49312871321823148)),
        ("floatTest".to_string(), NbtValue::Float(0.49823147058486938)),
        ("longTest".to_string(), NbtValue::Long(9223372036854775807)),
        ("listTest (compound)".to_string(), NbtValue::List(vec![
            NbtValue::Compound(None, vec![
                ("created-on".to_string(), NbtValue::Long(1264099775885)),
                ("name".to_string(), NbtValue::String("Compound tag #0".to_string())),
            ].into_iter().collect()),
            NbtValue::Compound(None, vec![
                ("created-on".to_string(), NbtValue::Long(1264099775885)),
                ("name".to_string(), NbtValue::String("Compound tag #1".to_string())),
            ].into_iter().collect()),
        ])),
        ("shortTest".to_string(), NbtValue::Short(32767)),
        ("byteArrayTest (the first 1000 values of (n*n*255+n*7)%100, starting with n=0 (0, 62, 34, 16, 8, ...))".to_string(), NbtValue::ByteArray(
            (0u128..1000u128).map(|n| (n*n*255+n*7) % 100).map(|i| i as i8).collect()
        )),
        ("nested compound test".to_string(), NbtValue::Compound(None, vec![
            ("ham".to_string(), NbtValue::Compound(None, vec![
                ("name".to_string(), NbtValue::String("Hampus".to_string())),
                ("value".to_string(), NbtValue::Float(0.75)),
            ].into_iter().collect())),
            ("egg".to_string(), NbtValue::Compound(None, vec![
                ("name".to_string(), NbtValue::String("Eggbert".to_string())),
                ("value".to_string(), NbtValue::Float(0.5)),
            ].into_iter().collect())),
        ].into_iter().collect())),
    ].into_iter().collect();
    let expected = NbtValue::Compound(Some("Level".to_string()), expected_values);
    assert_eq!(value, expected);
}

#[test]
fn test_player_nan_value() {
    let file = include_bytes!("../test_data/Player-nan-value.dat");
    let file = file.to_vec();
    let reader = NbtReader::from_compressed_data(file);
    let value = Java::from_reader(reader);
    let expected = NbtValue::Compound(Some("".to_string()), vec![
        ("Motion".to_string(), NbtValue::List(vec![
            NbtValue::Double(0.0),
            NbtValue::Double(0.0),
            NbtValue::Double(0.0),
        ])),
        ("FallDistance".to_string(), NbtValue::Float(0.0)),
        ("Pos".to_string(), NbtValue::List(vec![
            NbtValue::Double(0.0),
            NbtValue::Double(f64::NAN),
            NbtValue::Double(0.0),
        ])),
        ("Health".to_string(), NbtValue::Short(20)),
        ("DeathTime".to_string(), NbtValue::Short(0)),
        ("Fire".to_string(), NbtValue::Short(-20)),
        ("Air".to_string(), NbtValue::Short(300)),
        ("OnGround".to_string(), NbtValue::Byte(1)),
        ("HurtTime".to_string(), NbtValue::Short(0)),
        ("Rotation".to_string(), NbtValue::List(vec![
            NbtValue::Float(164.39995),
            NbtValue::Float(-63.150204),
        ])),
        ("AttackTime".to_string(), NbtValue::Short(0)),
        ("Inventory".to_string(), NbtValue::List(vec![]))
    ].into_iter().collect());
    println!("{:?}", value);
    assert!(value.is_ok());
    let value = value.unwrap();
    assert_eq!(value, expected);
}
