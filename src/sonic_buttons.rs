use crate::{ProcessHandleExt, B_POSITIVE_EDGE_TABLE, B_PRESSED_TABLE, X_POSITIVE_EDGE_TABLE, Y_POSITIVE_EDGE_TABLE, Y_PRESSED_TABLE};
use crate::process_reader::ProcessHandle;

pub fn separate_sonic(handle: ProcessHandle) {
    unsafe {
        // make first bounce only work with B
        handle.write_copy(0x0072527d, &B_POSITIVE_EDGE_TABLE).unwrap();
        // same, but with second bounce
        handle.write_copy(0x007252dc, &B_POSITIVE_EDGE_TABLE).unwrap();

        // make grinding only work with B
        handle.write_copy(0x007260c2, &B_PRESSED_TABLE).unwrap();

        // this replaces an omnibutton table with B table for spindashing
        handle.write_copy(0x00723ddb, &B_POSITIVE_EDGE_TABLE).unwrap();
        // this makes the above check jump straight to 
        handle.write_jump(0x00723de2, 0x00723e21 as *const fn()).unwrap();
        // make unrolls use only B
        handle.write_copy(0x0071a0fa, 0x00000002).unwrap();

        // if the above check doesn't give results, check with Y for somersault
        handle.write_copy(0x00723df8, &Y_POSITIVE_EDGE_TABLE).unwrap();
        // a jump for if we do get it
        handle.write_jump(0x00723e00, 0x0723ec0 as *const fn()).unwrap();
        // change an offset for if we don't get it
        handle.write_copy(0x00723dff, 0x5bu8).unwrap();

        // make light attack only work when releasing B
        handle.write_copy(0x00723106, 0x00000002).unwrap();

        // these are for somersault stuff

        // make it so that queued actions only work with X
        handle.write_copy(0x007230f1, &X_POSITIVE_EDGE_TABLE).unwrap();
        // make somersault continuation use Y
        handle.write_copy(0x0072393b, &Y_POSITIVE_EDGE_TABLE).unwrap();
        handle.write_copy(0x00723c9b, &Y_POSITIVE_EDGE_TABLE).unwrap();
        handle.write_copy(0x00723a5c, &Y_POSITIVE_EDGE_TABLE).unwrap();
        handle.write_copy(0x00723b72, &Y_POSITIVE_EDGE_TABLE).unwrap();
        handle.write_copy(0x0072394d, &Y_PRESSED_TABLE).unwrap();
        handle.write_copy(0x00723cae, &Y_PRESSED_TABLE).unwrap();
        handle.write_copy(0x00723a6e, &Y_PRESSED_TABLE).unwrap();
        handle.write_copy(0x00723b85, &Y_PRESSED_TABLE).unwrap();
    }
}
