#![no_std]
#![no_main]

use core::mem::MaybeUninit;

use flash_algorithm::*;
use rtt_target::{rprintln, rtt_init_print};

struct Algorithm;

algorithm!(Algorithm, {
    flash_address: 0x0,
    flash_size: 0x100000,
    page_size: 0x200,
    empty_value: 0xFF,
    sectors: [{
        size: 0x2000,
        address: 0x0,
    }]
});

impl FlashAlgorithm for Algorithm {
    fn new(_address: u32, _clock: u32, _function: Function) -> Result<Self, ErrorCode> {
        rtt_init_print!();
        rprintln!("Init");
        // TODO: Add setup code for the flash algorithm.
        Ok(Self)
    }

    fn erase_all(&mut self) -> Result<(), ErrorCode> {
        rprintln!("Erase All");
        // TODO: Add code here that erases the entire flash.
        Err(ErrorCode::new(0x70d0).unwrap())
    }

    fn erase_sector(&mut self, address: u32) -> Result<(), ErrorCode> {
        rprintln!("Erase sector address:{}", address);
        // TODO: Add code here that erases a page to flash.
        Ok(())
    }

    fn program_page(&mut self, address: u32, size: u32, _data: *const u8) -> Result<(), ErrorCode> {
        rprintln!("Program Page address:{} size:{}", address, size);
        // TODO: Add code here that writes a page to flash.
        Ok(())
    }
}

impl Drop for Algorithm {
    fn drop(&mut self) {
        // TODO: Add code here to uninitialize the flash algorithm.
    }
}
