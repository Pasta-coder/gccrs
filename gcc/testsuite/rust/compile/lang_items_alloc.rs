// { dg-do compile }
// { dg-options "-fdump-tree-gimple" }

#![feature(no_core, lang_items)]
#![no_core]

#[lang = "sized"]
pub trait Sized {}

#[lang = "owned_box"]
pub struct Box<T>(*mut T);

#[lang = "exchange_malloc"]
pub unsafe fn exchange_malloc(_size: usize, _align: usize) -> *mut u8 {
    0 as *mut u8
}

pub unsafe fn test_alloc() {
    let _ptr = exchange_malloc(4, 4);
}

// { dg-final { scan-tree-dump-times "exchange_malloc" 2 "gimple" } }
