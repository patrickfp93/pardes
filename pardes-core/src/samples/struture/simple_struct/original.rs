#[seferize::stringify(SIMPLE_STRUCT_SAMPLE)]
pub struct SimpleStruct {
    #[seferize::stringify(FIELD_1_SIMPLE_STRUCT_SAMPLE)]
    pub(super) field1: String,
    #[seferize::stringify(FIELD_2_SIMPLE_STRUCT_SAMPLE)]
    pub field2: i32,
}