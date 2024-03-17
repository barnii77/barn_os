/// Convert a u32 to a &str that lives on the stack.
/// This is important because this project is an operating system
/// and we don't have a heap allocator.
#[macro_export]
macro_rules! stack_cstr_from_num {
    ($x:expr, $t:ty) => {{
        // determine max num of digits in base 10
        // formula: ceil(log10(2^(8 * size_of<T>())))
        const MAX_LEN: usize = (<$t>::MAX.ilog10() + 1) as usize;
        let mut s = [0u8; MAX_LEN];
        let mut i = MAX_LEN - 1;
        let mut n = $x;
        while n > 0 && i > 0 {
            s[i] = (n % 10) as u8;
            n /= 10;
            i -= 1;
        }
        s
    }};
}
