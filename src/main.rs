use std::os::unix::io::{BorrowedFd, OwnedFd};

extern "C" {
    fn close(fd: OwnedFd) -> i32;

    fn ftruncate(fd: BorrowedFd, size: i64) -> i32;
}

// windows
use std::os::windows::io::{BorrowedHandle, OwnedHandle};

extern "C" {
    fn CloseHandle(handle: OwnedHandle) -> bool;

    fn SetEndOfFile(handle: BorrowedHandle) -> bool;
}

fn main() {
    todo!();
}
