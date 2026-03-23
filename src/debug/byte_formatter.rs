pub enum DebugFormat {
    Binary,
    Hex,
    Decimal,
}

// Use a trait that returns a slice to avoid Vec allocations
pub trait ToDebugBytes {
    fn to_debug_bytes(&self) -> Vec<u8>;
}

// Implement for u8 (no to_be_bytes needed)
impl ToDebugBytes for u8 {
    fn to_debug_bytes(&self) -> Vec<u8> { vec![*self] }
}

// Use a macro to implement for all other numeric types
macro_rules! impl_debug_bytes {
    ($($t:ty),*) => {
        $(
            impl ToDebugBytes for $t {
                fn to_debug_bytes(&self) -> Vec<u8> {
                    self.to_be_bytes().to_vec()
                }
            }
        )*
    };
}

impl_debug_bytes!(u16, u32, u64, u128, i16, i32, i64, i128);

// Implement for any size array (512-bit, etc)
impl<const N: usize> ToDebugBytes for [u8; N] {
    fn to_debug_bytes(&self) -> Vec<u8> {
        self.to_vec()
    }
}

pub fn print_state<T: ToDebugBytes>(label: &str, state: T, format: DebugFormat) {
    let bytes = state.to_debug_bytes();

    let output = match format {
        DebugFormat::Binary => {
            bytes.iter()
                .map(|b| format!("{:08b}", b))
                .collect::<Vec<String>>()
                .join(" ")
        }
        DebugFormat::Hex => {
            let body = bytes.iter()
                .map(|b| format!("{:02X}", b))
                .collect::<Vec<String>>()
                .join(" ");
            format!("0x{}", body)
        }
        DebugFormat::Decimal => format!("{:?}", bytes),
    };

    println!("{}: {}", label, output);
}