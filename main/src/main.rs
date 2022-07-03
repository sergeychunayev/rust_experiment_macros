use std::thread;
use std::time::Duration;

use time_proc_macro_attribute::timed;

fn main() {
    f();
    f_int();
    f_args(1, "hi");
    no_return();
    println!("recursion = {}", recursion(0));
}

#[timed]
fn f() -> String {
    thread::sleep(Duration::from_millis(500));
    return String::from("hi");
}

#[timed]
fn f_int() -> i64 {
    return 42
}

#[timed]
fn f_args(a: i64, b: &str) -> String {
    return format!("a={a}, b={b}", a = a, b = b)
}

#[timed]
fn no_return() {
    println!("no return");
}

#[timed]
fn recursion(x: i64) -> i64 {
    if x < 10 {
        return recursion(x + 1) + recursion(       x + 2)
    }
    return x
}
