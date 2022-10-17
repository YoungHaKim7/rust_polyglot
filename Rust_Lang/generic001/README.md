# LLDB

```
cargo run

cargo run --releases
```

2. cd target/debug 폴더로 이동

```
$ lldb generic001

(lldb) target create "generic001"
Current executable set to '/Users/globalyoung/Documents/Project/Github/rust_project/rust_polyglot/Rust_Lang/generic001/target/debug/generic001' (arm64).

(lldb) r
Process 4376 launched: '/Users/globalyoung/Documents/Project/Github/rust_project/rust_polyglot/Rust_Lang/generic001/target/debug/generic001' (arm64)
[(-10, -1000), (-9, -729), (-8, -512), (-7, -343), (-6, -216), (-5, -125), (-4, -64), (-3, -27), (-2, -8), (-1, -1), (0, 0), (1, 1), (2, 8), (3, 27), (4, 64), (5, 125), (6, 216), (7, 343), (8, 512), (9, 729)]
Process 4376 exited with status = 0 (0x00000000)


(lldb) b main
Breakpoint 1: 2 locations.


(lldb) list 2
   2
   3   	fn y_xxx<T>(x: T) -> T
   4   	where
   5   	    T: Mul<Output = T> + Copy,
   6   	{
   7   	    x * x * x
   8   	}
   9
   10  	fn main() {
   11  	    let my_vec = (-10..10).map(|x| (x, y_xxx(x))).collect::<Vec<_>>();

(lldb) r
Process 4402 launched: '/Users/globalyoung/Documents/Project/Github/rust_project/rust_polyglot/Rust_Lang/generic001/target/debug/generic001' (arm64)
Process 4402 stopped
* thread #1, queue = 'com.apple.main-thread', stop reason = breakpoint 1.2
    frame #0: 0x0000000100008af8 generic001`main
generic001`main:
->  0x100008af8 <+0>:  stp    x29, x30, [sp, #-0x10]!
    0x100008afc <+4>:  mov    x29, sp
    0x100008b00 <+8>:  mov    x2, x1
    0x100008b04 <+12>: mov    x8, x0
Target 0: (generic001) stopped.
(lldb) register read
General Purpose Registers:
        x0 = 0x0000000000000001
        x1 = 0x000000016fdff188
        x2 = 0x000000016fdff198
        x3 = 0x000000016fdff348
        x4 = 0x0000000000000000
        x5 = 0x0000000000000000
        x6 = 0x0000000000000000
        x7 = 0x0000000000000000
        x8 = 0x00000001000e4070  dyld`dyld4::sConfigBuffer
        x9 = 0x0000000000000002
       x10 = 0x0000000000000000
       x11 = 0x0000000000000002
       x12 = 0x0000000000000002
       x13 = 0x0000000000000000
       x14 = 0x0000000000000140
       x15 = 0x0000000000000000
       x16 = 0x000000030019b088
       x17 = 0x6ae100016fdfe3f0
       x18 = 0x0000000000000000
       x19 = 0x0000000100138060
       x20 = 0x0000000100008af8  generic001`main
       x21 = 0x00000001000e4070  dyld`dyld4::sConfigBuffer
       x22 = 0x0000000000000000
       x23 = 0x0000000000000000
       x24 = 0x0000000000000000
       x25 = 0x0000000000000000
       x26 = 0x0000000000000000
       x27 = 0x0000000000000000
       x28 = 0x0000000000000000
        fp = 0x000000016fdff160
        lr = 0x000000010008908c  dyld`start + 520
        sp = 0x000000016fdff020
        pc = 0x0000000100008af8  generic001`main
      cpsr = 0x60001000

(lldb)

(lldb) gui

(lldb) exit
Quitting LLDB will kill one or more processes. Do you really want to proceed: [Y/n] y

```
