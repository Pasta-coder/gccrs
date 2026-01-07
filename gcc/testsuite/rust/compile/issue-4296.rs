// { dg-options "-w" }
pub fn main() {
    let x = 3;

    match x { // { dg-error "non-exhaustive patterns" }
        4 => 0,   
        3 => 0, 
    };
}
