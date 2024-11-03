struct Wrapper(i32);

fn take_ownership(_: Wrapper) {}

fn take_mut_ref(x : &mut Wrapper , i :i32) -> &mut Wrapper{
    if i <= 0 {
        return x
    }
    take_mut_ref(x, i-1);
    x
}


fn main() {
    let mut x = Wrapper(42);
    take_mut_ref(&mut x, 5);
    take_ownership(x); // x is moved here
    let data = "fbrns";
    let mut z = data.to_string();
    let q = String::from("lol");
    z.push_str(&q);
    println!("{} {}", z, q);
    
    //println!("{}", x.0); // Error: value borrowed here after move
}