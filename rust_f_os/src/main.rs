
#![no_std] // 不链接Rust标准库
#![no_main] // 禁用所有Rust层级的入口点

use core::panic::PanicInfo;
mod vga_buffer;
/// 这个函数将在panic时被调用
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

static HELLO: &[u8] = b"Hello World!";


#[no_mangle]
pub extern "C" fn _start() -> ! {
    //0xb8000是显存的起始地址，0xb8000-0xb8f9f 这段内存空间写入数据会显示在屏幕上
    let vga_buffer = 0xb8000 as *mut u8;

    /*
    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
    */
   // panic!("Some panic message");
    println!("中文 World{}", "!");
    loop {}
}