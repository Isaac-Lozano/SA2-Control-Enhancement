use crate::{B_POSITIVE_EDGE_TABLE, X_POSITIVE_EDGE_TABLE, B_PRESSED_TABLE};
use crate::process_reader::ProcessHandle;

pub fn separate_tails(handle: ProcessHandle) {
    unsafe {
        // make cannon use only B
        handle.write_copy(0x0074162c, &B_POSITIVE_EDGE_TABLE).unwrap();
        handle.write_copy(0x00741878, &B_PRESSED_TABLE).unwrap();
        handle.write_copy(0x00741988, &B_PRESSED_TABLE).unwrap();
        handle.write_copy(0x007419c2, &B_PRESSED_TABLE).unwrap();

        // have queued actions work with X
        handle.write_copy(0x00749749, &X_POSITIVE_EDGE_TABLE).unwrap();
    }
}
