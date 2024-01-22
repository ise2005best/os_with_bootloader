#![no_std]
#![no_main]

mod writer;
use bootloader_api::config::{Mapping, FrameBuffer};
use x86_64::instructions::hlt;
use writer::FrameBufferWriter;
use core::panic::PanicInfo;

// configurations for the bootloader
pub static BOOTLOADER_CONFIG: bootloader_api::BootloaderConfig ={
    let mut config = bootloader_api::BootloaderConfig::new_default();
    config.mappings.physical_memory = Some((Mapping::Dynamic)); // makes memory allocation for the kernel dynamic
    config.kernel_stack_size = 100 * 1024; // allocates 100kb to the kernel stack
    config
};
#[macro_export]
macro_rules! println {
    ($fb_writer:expr, $($arg:tt)*) => {
        {
            use core::fmt::Write;
            let _ = write!($fb_writer, $($arg)*);
        }
    };
}
bootloader_api::entry_point!(my_entry_point, config = &BOOTLOADER_CONFIG); // informs compiler of my bootloader entry points and the configuration of my bootloader

fn my_entry_point(boot_info: &'static mut bootloader_api::BootInfo) -> ! {

    let frame_buffer_info = boot_info.framebuffer.as_mut().unwrap().info();

    let buffer = boot_info.framebuffer.as_mut().unwrap().buffer_mut();

    let mut frame_buffer_writer = FrameBufferWriter::new(buffer, frame_buffer_info, 0, 0);
    // let mut frame_buffer_writer2 = FrameBufferWriter2::new(buffer, frame_buffer_info);

    // use core::fmt::Write;//below requires this
    println!(frame_buffer_writer, "Testing testing  {} and  {} using println", 1, 4.0/2.0);
    frame_buffer_writer.setPosition(50, 100);
    println!(frame_buffer_writer, "Testing testing {} and {} using println! after setting position", 1, 4.0/2.0);
    frame_buffer_writer.setPosition(100, 200);
    println!(frame_buffer_writer, "Testing testing {} and {} using println! after setting position again", 1, 4.0/2.0);
    // writeln!(frame_buffer_writer, "Testing testing {} and  {} using println! after setting position", 1, 4.0/2.0).unwrap();

    loop {
        hlt(); //stop x86_64 from being unnecessarily busy while looping
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) ->!{
    loop {
        hlt();
    }
}