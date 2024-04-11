use entmet_macros::TestEntityTraitImpl;
pub trait EntityTraitTestTrait {
    fn do_something(&self) -> &str;
    fn do_more(&self) -> &str;
    fn add(&self, a: u8, b: u8) -> u8;
}
pub struct Entity;
impl EntityTraitTestTrait for Entity {
    fn do_something(&self) -> &str {
        "Hello, World!"
    }
    fn do_more(&self) -> &str{
        "More!"
    }
    fn add(&self, a: u8, b: u8) -> u8 {
        a + b
    }
}
#[derive(TestEntityTraitImpl)]
struct TestStruct {
    entity: Entity,
}

#[test]
fn test_test_trait_implementation() {
    let ts = TestStruct { entity: Entity };
    assert_eq!(ts.do_something(), "Hello, World!");
    assert_eq!(ts.do_more(), "More!");
    assert_eq!(ts.add(1, 2), 3);
}
