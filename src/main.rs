#![no_std]
#![no_main]

use core::mem::MaybeUninit;

use atsamd_hal::{
    nvm::{self, Nvm, WriteGranularity},
    pac,
};
use flash_algorithm::*;

#[cfg(not(feature = "log"))]
macro_rules! rprintln {
    ($($arg:tt)*) => {
        {
            let _ = ($($arg)*);
        }
    }
}

#[cfg(feature = "log")]
macro_rules! rprintln {
    ($($arg:tt)*) => {
        rtt_target::rprintln!($($arg)*)
    }
}

struct Algorithm {
    nvm: Nvm,
}

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

impl Algorithm {
    fn init(_address: u32, _clock: u32, function: Function) -> Result<Self, nvm::Error> {
        // Safety: the only place `Peripherals` struct is accessed from
        // Note: `pac::Peripherals::take` can be only called once whilst `Self::init` is
        // called multiple times during the program lifetime. Thus it cannot be used.
        let device = unsafe { pac::Peripherals::steal() };
        const NVMCTRL_ID: u16 = 0x22;
        // Unlock NVMCTRL peripheral if it was locked
        // Safety: NVMCTRL_ID is a valid peripheral identifier
        device
            .PAC
            .wrctrl
            .write(|w| unsafe { w.key().clr().perid().bits(NVMCTRL_ID) });

        let mut nvm = Nvm::new(device.NVMCTRL);

        if function != Function::Verify as _ {
            if cfg!(feature = "override-boot-loader-protection") {
                rprintln!("Disable BOOTPROT");
                nvm.boot_protection(false)?;
            }

            rprintln!("Disable region lock");
            nvm.region_lock(0xFFFF_FFFF)?;
        }
        Ok(Self { nvm })
    }
}

impl FlashAlgorithm for Algorithm {
    fn new(address: u32, clock: u32, function: Function) -> Result<Self, ErrorCode> {
        #[cfg(feature = "log")]
        rtt_target::rtt_init_print!();
        rprintln!("Init");
        Self::init(address, clock, function).map_err(|e| {
            rprintln!("init failed: {:?}", e);
            ErrorCode::new(1).unwrap()
        })
    }

    fn erase_sector(&mut self, address: u32) -> Result<(), ErrorCode> {
        rprintln!("Erase sector address:{}", address);
        // Safety: The flash programming algorithm is not meant to be loaded to flash,
        // so the executing code will not be erased.
        unsafe { self.nvm.erase_flash(address as *mut u32, 1) }.map_err(|e| {
            rprintln!("erase_sector failed: {:?}", e);
            ErrorCode::new(1).unwrap()
        })
    }

    fn program_page(&mut self, address: u32, data: &[u8]) -> Result<(), ErrorCode> {
        rprintln!("Program Page address:{} size:{}", address, data.len());
        // Include any unaligned trailing bytes
        let size_words = (data.len() as u32 + 3) / 4;
        // Safety: Trust that the host asked for the right thing.
        unsafe {
            self.nvm.write_flash(
                address as *mut _,
                data.as_ptr() as *const _,
                size_words,
                WriteGranularity::Page,
            )
        }
        .map_err(|e| {
            rprintln!("program_page failed: {:?}", e);
            ErrorCode::new(1).unwrap()
        })
    }
}

impl Drop for Algorithm {
    fn drop(&mut self) {
        rprintln!("Uninit");
    }
}
