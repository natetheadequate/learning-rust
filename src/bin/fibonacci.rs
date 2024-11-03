use std::{
    fmt,
    ops::Add
};

enum Nat {
    Zero(),
    Succ(Box<Nat>),
}

impl Nat {
    fn new(n: u64) -> Nat {
        let mut builder = Nat::Zero();
        for _ in 0..n {
            builder = Nat::Succ(Box::new(builder))
        }
        return builder;
    }
}

fn nat_to_int(nat: &Nat) -> u64 {
    match nat {
        Nat::Zero() => 0,
        Nat::Succ(n) => 1 + nat_to_int(n),
    }
}

impl fmt::Display for Nat {
    fn fmt(self: &Nat, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Nat({})", nat_to_int(self))
    }
}

impl Add<Nat> for Nat {
    type Output = Nat;
    fn add(self, rhs: Nat) -> Self::Output {
        match self {
            Nat::Zero() => rhs,
            Nat::Succ(n) => *n + Nat::Succ(Box::new(rhs)),
        }
    }
}

impl PartialEq<u64> for Nat {
    fn eq(&self, other: &u64) -> bool {
        nat_to_int(self) == *other
    }
}

trait RefSub {
    fn ref_sub(&self, rhs : u64) -> Self;
}

impl RefSub for Nat {
    fn ref_sub(&self, rhs : u64) -> Self {
        Nat::new(nat_to_int(self)-rhs)
    }
}

impl RefSub for u64 {
    fn ref_sub(&self, rhs : u64) -> Self {
        self - rhs
    }
}

fn fib<T>(n: T) -> T
where
    T: PartialEq<u64> + Add<T, Output = T> + RefSub
{
    if n == 0 || n == 1 {
        n
    } else {
        fib(n.ref_sub(1)) + fib(n.ref_sub(2))
    }
}

pub fn run_fibonacci() {
    println!("{}", fib(0));
    println!("{}", fib(1));
    println!("{}", fib(10));
    println!("{}", Nat::Succ(Box::new(Nat::Succ(Box::new(Nat::Zero())))));
    println!("{}", Nat::new(10));
    println!("{}", Nat::new(10) + Nat::new(5));
    println!("{}", Nat::new(10).ref_sub(2));
    println!("{}", fib(Nat::new(10)));
}
