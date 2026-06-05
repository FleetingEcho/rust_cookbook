fn main() {
    // TODO: Create an array called `a` with at least 100 elements in it.
    // let a:u16[]=[10].repeat(100)
    let a: [u16; 100] = [10; 100];
    // let a: Vec<u16> = [10].repeat(100);
    // 或更简洁
    // let a = vec![10u16; 100];;

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed");
    }
}
