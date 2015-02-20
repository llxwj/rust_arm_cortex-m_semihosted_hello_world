#![crate_type="rlib"]
#![no_std]
#![feature(intrinsics)]
#![allow(unstable)]
#![feature(asm)]
#![feature(lang_items)]

#[lang="sized"]
trait Sized { }

#[lang="copy"]
trait Copy { }

#[lang="sync"]
trait Sync { }

#[link_section=".isr_vector"]
pub static ISRVECTORS: [unsafe extern "C" fn(); 1] = [
    main, 
];

fn semihosting(command: u32, message: &[u32; 3]) {
    unsafe {
        asm!(
            "mov r0, $0;
             mov r1, $1;
             bkpt #0xAB"
            : 
            : "r"(command), "r"(message)
            : "r0", "r1"
        );
    }
}

extern "rust-intrinsic" {
    fn uninit<T>() -> T;
    fn copy_nonoverlapping_memory<T>(dst: *mut T, src: *const T, count: usize);
}

#[inline]
unsafe fn uninitialized<T>() -> T {
    uninit()
}

#[inline(always)]
unsafe fn read<T>(src: *const T) -> T {
    let mut tmp: T = uninitialized();
    copy_nonoverlapping_memory(&mut tmp, src, 1);
    tmp
}

#[inline]
unsafe fn transmute_copy<T, U>(src: &T) -> U {
    read(src as *const T as *const U)
}

#[repr(C)]
#[allow(dead_code)]
struct Slice<T> {
    data: *const T,
    len: usize,
}

unsafe trait Repr<T> {
    #[inline]
    fn repr(&self) -> T { 
        unsafe { 
            transmute_copy(&self) 
        } 
    }
}

unsafe impl Repr<Slice<u8>> for str {}

#[no_mangle]
pub extern "C" fn main() {
    let text = "Hello World!\n";
    
    let message : [u32; 3] = [
        2, //stderr
        text.repr().data as u32,
        text.repr().len as u32
    ];

    loop { 
        semihosting(5, &message);
    }
}