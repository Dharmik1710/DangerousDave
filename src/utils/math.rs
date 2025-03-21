use std::convert::TryInto;

fn convert_to_u32(x: i32) -> Option<u32> {
    x.try_into().ok()
}
