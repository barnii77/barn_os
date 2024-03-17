use core::panic::PanicInfo;
use crate::stack_cstr_from_num;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // print the panic message to vga buffer
    let (file, line) = if let Some(location) = info.location() {
        (location.file(), location.line())
    } else {
        ("<unknown file>", u32::MAX)
    };
    let message = if let Some(s) = info.payload().downcast_ref::<&str>() {
        s
    } else {
        "<unknown panic reason>"
    };
    crate::vga_buffer::print_white_on_black("panic at ".bytes());
    crate::vga_buffer::print_white_on_black(file.bytes());
    crate::vga_buffer::print_white_on_black(":".bytes());
    let line_cstr = stack_cstr_from_num!(line, u32);
    crate::vga_buffer::print_white_on_black(line_cstr.into_iter());
    crate::vga_buffer::print_white_on_black(": ".bytes());
    crate::vga_buffer::print_white_on_black(message.bytes());
    loop {}
}
