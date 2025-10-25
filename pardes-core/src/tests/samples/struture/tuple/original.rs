use seferize::stringify;

#[stringify("TUPLE_SAMPLE")]
pub struct Tuple(#[stringify(FIELD_1_TUPLE_SAMPLE)]pub(super) String,#[stringify(FIELD_2_TUPLE_SAMPLE)] pub i32);
