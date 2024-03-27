use crate::{NbtData, NbtValue, error::NbtResult};

#[test]
fn test() {
    let file = include_bytes!("./neccessary_test_files/hello_world.nbt");
    let mut data = NbtData::from_uncompressed_data(file.to_vec()).unwrap();
    let test = crate::traits::filesystem::read_root_compound(&mut data);
    let expected: NbtResult<NbtValue> = Ok(NbtValue::Compound((Some("hello world".to_string()), vec![("name".to_string(), NbtValue::String("Bananrama".to_string()))])));
    println!("data: {:?}", test);
    assert_eq!(test, expected);
}
