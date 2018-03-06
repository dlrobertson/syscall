use super::error::{Error, Result};

pub unsafe fn syscall0(mut a: usize) -> Result<usize> {
    asm!("syscall"
        : "={rax}"(a)
        : "{rax}"(a)
        : "memory" "rcx" "r11"
        : "intel", "volatile");

    Error::demux(a)
}

pub unsafe fn syscall1(mut a: usize, b: usize) -> Result<usize> {
    asm!("syscall"
        : "={rax}"(a)
        : "{rax}"(a), "{rdi}"(b)
        : "memory" "rcx" "r11"
        : "intel", "volatile");

    Error::demux(a)
}

// Clobbers all registers - special for clone
pub unsafe fn syscall1_clobber(mut a: usize, b: usize) -> Result<usize> {
    asm!("syscall"
        : "={rax}"(a)
        : "{rax}"(a), "{rdi}"(b)
        : "memory", "rbx", "rcx", "rdx", "rsi", "rdi", "r8",
          "r9", "r10", "r11", "r12", "r13", "r14", "r15"
        : "intel", "volatile");

    Error::demux(a)
}

pub unsafe fn syscall2(mut a: usize, b: usize, c: usize) -> Result<usize> {
    asm!("syscall"
        : "={rax}"(a)
        : "{rax}"(a), "{rdi}"(b), "{rsi}"(c)
        : "memory" "rcx" "r11"
        : "intel", "volatile");

    Error::demux(a)
}

pub unsafe fn syscall3(mut a: usize, b: usize, c: usize, d: usize) -> Result<usize> {
    asm!("syscall"
        : "={rax}"(a)
        : "{rax}"(a), "{rdi}"(b), "{rsi}"(c), "{rdx}"(d)
        : "memory" "rcx" "r11"
        : "intel", "volatile");

    Error::demux(a)
}

pub unsafe fn syscall4(mut a: usize, b: usize, c: usize, d: usize, e: usize) -> Result<usize> {
    asm!("syscall"
        : "={rax}"(a)
        : "{rax}"(a), "{rdi}"(b), "{rsi}"(c), "{rdx}"(d), "{rcx}"(e)
        : "memory" "rcx" "r11"
        : "intel", "volatile");

    Error::demux(a)
}

pub unsafe fn syscall5(mut a: usize, b: usize, c: usize, d: usize, e: usize, f: usize)
                       -> Result<usize> {
    asm!("syscall"
        : "={rax}"(a)
        : "{rax}"(a), "{rdi}"(b), "{rsi}"(c), "{rdx}"(d), "{rcx}"(e), "{r8}"(f)
        : "memory" "rcx" "r11"
        : "intel", "volatile");

    Error::demux(a)
}
