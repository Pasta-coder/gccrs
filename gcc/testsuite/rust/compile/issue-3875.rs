// { dg-options "-w" }

#[derive(Clone)] // { dg-error "derive may only be applied to structs, enums and unions" }
fn foo() {}      // { dg-excess-errors "" }

#[derive(Debug)] // { dg-error "derive may only be applied to structs, enums and unions" }
trait Bar {}     // { dg-excess-errors "" }

#[derive(Copy, Clone)]
struct TupleStruct(i32); // This should NOT error

fn main() {}
