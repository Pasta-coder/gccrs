// { dg-options "-w" }
#[target_feature(enable = "sse2")] // { dg-error "attribute can only be applied to .unsafe. functions" }
fn foo() {}