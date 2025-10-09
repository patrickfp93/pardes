pub struct SimpleStruct {
    pub(super) field1: String,
    //#[only_read]
    pub field2: i32,
}