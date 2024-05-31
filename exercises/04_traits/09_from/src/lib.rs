// TODO: Implement the `From` trait for the `WrappingU32` type to make `example` compile.

#[allow(unused_variables)]
pub struct WrappingU32 {
    value: u32,
}

#[allow(unused_variables)]
impl From<u32> for WrappingU32 {
    fn from(s: u32) -> WrappingU32 {
        WrappingU32 {
            value: s
        }
    }
}

#[allow(unused_variables)]
fn example() {
    let wrapping: WrappingU32 = 42.into();
    let wrapping = WrappingU32::from(42);
}
