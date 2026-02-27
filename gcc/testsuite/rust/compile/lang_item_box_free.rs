// { dg-do compile }
// { dg-options "-fdump-tree-gimple" }

#![feature(no_core, lang_items)]
#![no_core]

#[lang = "sized"]
pub trait Sized {}

#[lang = "owned_box"]
pub struct Box<T>(*mut T);

// The global deallocation hook for Box
#[lang = "box_free"]
pub unsafe fn box_free<T>(_ptr: *mut T) {
    // Dummy drop implementation
}

pub unsafe fn test_drop(_ptr: *mut u8) {
    // Force the compiler to resolve the lang item
    box_free(_ptr);
}

// Verify the frontend successfully passed the lang item function to the backend
// { dg-final { scan-tree-dump-times "box_free" 2 "gimple" } }