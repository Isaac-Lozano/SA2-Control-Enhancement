#![feature(naked_functions)]
#![feature(asm)]

mod process_reader;
mod sonic_buttons;
mod knux_buttons;
mod tails_buttons;

use process_reader::ProcessHandle;

pub const BUTTONS_PRESSED_PTR: *mut u32 = 0x01defc08 as *mut u32;
pub const BUTTONS_POSITIVE_EDGE_PTR: *mut u32 = 0x01defc10 as *mut u32;

pub static mut B_PRESSED_TABLE: [u8; 2] = [0; 2];
pub static mut B_POSITIVE_EDGE_TABLE: [u8; 2] = [0; 2];
pub static mut X_POSITIVE_EDGE_TABLE: [u8; 2] = [0; 2];
pub static mut Y_PRESSED_TABLE: [u8; 2] = [0; 2];
pub static mut Y_POSITIVE_EDGE_TABLE: [u8; 2] = [0; 2];

pub trait ProcessHandleExt {
    fn write_jump(&self, address: u32, function: *const fn()) -> Result<(), &'static str>;
    fn write_call(&self, address: u32, function: *const fn()) -> Result<(), &'static str>;
}

impl ProcessHandleExt for ProcessHandle {
    fn write_jump(&self, address: u32, function: *const fn()) -> Result<(), &'static str> {
        let function_address = function as u32;
        self.write_copy(address, 0xe9u8)?;
        self.write_copy(address + 1, function_address - (address + 5))
    }

    fn write_call(&self, address: u32, function: *const fn()) -> Result<(), &'static str> {
        let function_address = function as u32;
        self.write_copy(address, 0xe8u8)?;
        self.write_copy(address + 1, function_address - (address + 5))
    }
}

#[repr(C)]
pub struct ModInfo {
    version: u32,
    init: u32,
    padding: [u32; 8],
}

#[no_mangle]
pub static SA2ModInfo: ModInfo = ModInfo {
    version: 1,
    init: 0,
    padding: [0; 8],
};

extern "C" fn update_input_tables() {
    unsafe {
        if *BUTTONS_PRESSED_PTR & 0x2 != 0 {
            B_PRESSED_TABLE[0] = 1;
        } else {
            B_PRESSED_TABLE[0] = 0;
        }

        if *BUTTONS_POSITIVE_EDGE_PTR & 0x2 != 0 {
            B_POSITIVE_EDGE_TABLE[0] = 1;
        } else {
            B_POSITIVE_EDGE_TABLE[0] = 0;
        }

        if *BUTTONS_POSITIVE_EDGE_PTR & 0x400 != 0 {
            X_POSITIVE_EDGE_TABLE[0] = 1;
        } else {
            X_POSITIVE_EDGE_TABLE[0] = 0;
        }

        if *BUTTONS_POSITIVE_EDGE_PTR & 0x200 != 0 {
            Y_POSITIVE_EDGE_TABLE[0] = 1;
        } else {
            Y_POSITIVE_EDGE_TABLE[0] = 0;
        }

        if *BUTTONS_PRESSED_PTR & 0x200 != 0 {
            Y_PRESSED_TABLE[0] = 1;
        } else {
            Y_PRESSED_TABLE[0] = 0;
        }
    }
}

#[no_mangle]
pub extern "C" fn Init(_path: u32, _helper_functions: u32) {
    let handle = ProcessHandle::open_current_process();
    
    // add our tables at the end of the normal function
    handle.write_jump(0x00442480, update_input_tables as *const fn()).unwrap();

    sonic_buttons::separate_sonic(handle);
    knux_buttons::separate_knuckles(handle);
    tails_buttons::separate_tails(handle);

    // move prompt cycling to D-Pad up
    handle.write_copy(0x00794217, 0x10).unwrap();

    // set action window to only disappear when X is pressed
    handle.write_copy(0x00794229, 0x00000400).unwrap();
    handle.write_copy(0x0079529d, 0x00000400).unwrap();

    println!("Loaded SA2 mod");
}
