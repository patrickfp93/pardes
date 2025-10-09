use crate::tests::samples::struture::simple_struct::expected::SimpleStruct;

mod original;
mod expected;

//super context
#[test]
pub fn access_simple_struture(){
    let mut s_s = SimpleStruct::new("".into(),0);
    let field1 = s_s.field1().clone();
    s_s.sum_five();
    let field2 = *s_s.field2();
    assert_eq!(field1,"");
    assert_eq!(field2,5);
}