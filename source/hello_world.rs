// Copyright (c) 2015 Michael V. Franklin
//      
// This file is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
// 
// This file is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
// 
// You should have received a copy of the GNU General Public License
// along with this file.  If not, see <http://www.gnu.org/licenses/>.


#![crate_type="staticlib"]
#![feature(intrinsics)]
#![feature(no_std)]
#![no_std]
#![feature(asm)]
#![feature(lang_items)]

// Just to make things build
//------------------------------------------------------------------------------
#[lang="phantom_fn"]
trait PhantomFn<A:?Sized,R:?Sized=()> { }

#[lang="sized"]
trait Sized : PhantomFn<Self> {}

#[lang="copy"]
trait Copy : PhantomFn<Self>  { }

#[lang="sync"]
trait Sync : PhantomFn<Self>  { }

// Incomplete ISR vector table.  Just the reset handler to start execution at
// main
//------------------------------------------------------------------------------
#[link_section=".isr_vector"]
pub static ISRVECTORS: [unsafe extern "C" fn(); 1] = [
    main, 
];

// Inline assembly to invoke semihosting commands
//------------------------------------------------------------------------------
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

// Minimal Rust runtime implementation
//------------------------------------------------------------------------------
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

// Program entry point
//------------------------------------------------------------------------------
#[no_mangle]
pub extern "C" fn main() {
    let text = "Hello World!\n";
    
    let message : [u32; 3] = [
        2,               // write to stderr
        text.repr().data as u32,
        text.repr().len as u32
    ];

    loop { 
        semihosting(
            5,           // 5 is semihosting "write" command
            &message
        );
    }
}