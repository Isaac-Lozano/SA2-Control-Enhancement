use crate::{ProcessHandleExt, B_POSITIVE_EDGE_TABLE, X_POSITIVE_EDGE_TABLE, BUTTONS_POSITIVE_EDGE_PTR, BUTTONS_PRESSED_PTR, B_PRESSED_TABLE};
use crate::process_reader::ProcessHandle;

const CURRENT_LEVEL: *mut u32 = 0x01934b70 as *mut u32;

fn hunter_slow_punch_1_start(action_struct: u32, physics_struct: u32) {
    unsafe {
        asm!("
            mov $1, %edi
            mov $0, %eax
            push %edi
            call *$2
            add $$0x04, %esp
            "
            :
            : "r" (action_struct), "r" (physics_struct), "r" (0x00734e50)
            : "edi", "eax"
            :
        );
    }
}

fn hunter_fast_punch_1_start(action_struct: u32, physics_struct: u32) {
    unsafe {
        asm!("
            mov $1, %edi
            mov $0, %eax
            push %edi
            call *$2
            add $$0x04, %esp
            "
            :
            : "r" (action_struct), "r" (physics_struct), "r" (0x00734ec0)
            : "edi", "eax"
            :
        );
    }
}

fn hunter_spiral_upper(action_struct: u32, physics_struct: u32) {
    unsafe {
        asm!("
            mov $1, %ebx
            mov $0, %eax
            push %ebx
            call *$2
            add $$0x04, %esp
            "
            :
            : "r" (action_struct), "r" (physics_struct), "r" (0x00734f30)
            : "ebx", "eax"
            :
        );
    }
}

fn hunter_drill_dive(action_struct: u32, physics_struct: u32) {
    unsafe {
        asm!("
            mov $1, %ecx
            mov $0, %eax
            push %ecx
            call *$2
            add $$0x04, %esp
            "
            :
            : "r" (action_struct), "r" (physics_struct), "r" (0x00733d40)
            : "ecx", "eax"
            :
        );
    }
}

#[naked]
unsafe fn knuckles_action_end_hook() {
    asm!("
        call *$0
        pop %edi
        pop %esi
        pop %ebp
        pop %ebx
        pop %ecx
        ret
    "
    :
    : "r" (knuckles_additional_action_checks as *const fn() as u32)
    );
}

// this seems likely to break
// especially if the memory inputs decide to use the stack
#[naked]
unsafe fn drill_dive_check_hook() {
    asm!("
        push %eax
        call *$0
        test %eax, %eax
        pop %eax
        jz fail_return
        xor %ecx, %ecx
        test %ebx, %ebx
        jle fail_return
        jmp *$2

        fail_return:
        jmp *$1
    "
    :
    : "{ecx}" (extra_drill_dive_check as *const fn() -> u32 as u32), "m" (0x0073392c), "m" (0x00733919)
    );
}

unsafe extern "C" fn extra_drill_dive_check() -> u32 {
    if *BUTTONS_PRESSED_PTR & 0x2 != 0 {
        1
    } else {
        0
    }
}

// returns 1 if we change state, 0 otherwise
extern "C" fn knuckles_additional_action_checks() -> u32 {
    // TODO: 2-player
    unsafe {
        let handle = ProcessHandle::open_current_process();
        let action_struct_ptr = handle.read_copy(0x01dea6c0).unwrap();
        let physics_struct_ptr = handle.read_copy(0x01de9600).unwrap();

        let num_actions: u8 = handle.read_copy(physics_struct_ptr + 0xc).unwrap();
        if *BUTTONS_POSITIVE_EDGE_PTR & 0x2 != 0 {
            let mut selected_action: u8 = handle.read_copy(physics_struct_ptr + 0xd).unwrap();
            if num_actions != 0 {
                let mut action_idx: u32 = 0;
                while action_idx < num_actions as u32 {
                    let action = handle.read_copy(physics_struct_ptr + 0x4 + action_idx).unwrap();
                    if action == 0x47 || action == 0x4a || action == 0x4e {
                        selected_action = action;
                        break;
                    }
                    action_idx += 1;
                }
                if action_idx == num_actions as u32 {
                    selected_action = handle.read_copy(physics_struct_ptr + 0x4).unwrap();
                    if *CURRENT_LEVEL == 90 && (selected_action == 0x47 || selected_action == 0x4a || selected_action == 0x4d) {
                        return 0;
                    }
                }
            }
            if selected_action == 0x47 {
                hunter_slow_punch_1_start(action_struct_ptr, physics_struct_ptr);
            } else if selected_action == 0x4a {
                hunter_fast_punch_1_start(action_struct_ptr, physics_struct_ptr);
            } else if selected_action == 0x4e {
                hunter_drill_dive(action_struct_ptr, physics_struct_ptr);
            } else {
                return 0;
            }
            return 1;
        }
        if *BUTTONS_POSITIVE_EDGE_PTR & 0x200 != 0 {
            let action: u8 = handle.read_copy(action_struct_ptr).unwrap();
            if action == 0x0 || action == 0x1 || action == 0xc || action == 0x40 || action == 0x11 {
                hunter_spiral_upper(action_struct_ptr, physics_struct_ptr);
                return 1;
            } else {
                return 0;
            }
        }
    }
    0
}

pub fn separate_knuckles(handle: ProcessHandle) {
    // add a hook so that we can handle punching
    handle.write_jump(0x0073392c, knuckles_action_end_hook as *const fn()).unwrap();
    // add a hook so that we can control when we dig
    handle.write_jump(0x00733913, drill_dive_check_hook as *const fn()).unwrap();

    unsafe {
        // these allow us to continue punches using B
        handle.write_copy(0x00734aff, &B_POSITIVE_EDGE_TABLE).unwrap();
        handle.write_copy(0x00734cf5, &B_POSITIVE_EDGE_TABLE).unwrap();
        handle.write_copy(0x00734ba5, &B_POSITIVE_EDGE_TABLE).unwrap();
        handle.write_copy(0x00734c10, &B_POSITIVE_EDGE_TABLE).unwrap();
        handle.write_copy(0x00734b11, &B_PRESSED_TABLE).unwrap();
        handle.write_copy(0x00734d07, &B_PRESSED_TABLE).unwrap();
        handle.write_copy(0x00734bb7, &B_PRESSED_TABLE).unwrap();
        handle.write_copy(0x00734c22, &B_PRESSED_TABLE).unwrap();
        handle.write_copy(0x00734e1c, &B_PRESSED_TABLE).unwrap();

        // change some data in a jump table so that punch, spinput, and drill aren't triggered
        // by both X and B
        handle.write_copy(0x00733d21, 0x1fu8).unwrap();
        handle.write_copy(0x00733d24, 0x1fu8).unwrap();
        handle.write_copy(0x00733d27, 0x1fu8).unwrap();
        handle.write_copy(0x00733d28, 0x1fu8).unwrap();

        // make queued actions work with only X
        handle.write_copy(0x007338f6, &X_POSITIVE_EDGE_TABLE).unwrap();

        // skip spinput check because we just use Y for that
        handle.write_jump(0x00734a34, 0x00734a51 as *const fn()).unwrap();
    }
}
