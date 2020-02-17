#![allow(non_snake_case)]

fn main() {
    let mut a = [0.0; 8];
    let b = [1.0; 8];
    unsafe {
        println!("a[7] = {}", a.get_unchecked(7));
        println!("a[8] = {}", a.get_unchecked(8));
        *a.get_unchecked_mut(9) = 3.0;
        println!("b = {:?}", b);
        *a.get_unchecked_mut(10000000000) = 3.0;
        println!("done!");
    }
}
