const VGA_BUFFER_WIDTH: usize = 80;
const VGA_BUFFER_HEIGHT: usize = 25;
const VGA_BUFFER_SIZE_BYTES: usize = 2 * VGA_BUFFER_WIDTH * VGA_BUFFER_HEIGHT;
const VGA_BUFFER_POINTER: *mut u8 = 0xb8000usize as *mut _;

pub fn print(s: impl Iterator<Item = u8>, color: u8) {
    let mut offset = 0;
    for byte in s.take(VGA_BUFFER_SIZE_BYTES) {
        unsafe {
            *VGA_BUFFER_POINTER.offset(offset) = byte;
            *VGA_BUFFER_POINTER.offset(offset + 1) = color;
        }
        offset += 2;
    }
}

pub fn print_white_on_black(s: impl Iterator<Item = u8>) {
    print(s, 0x0f);
}
