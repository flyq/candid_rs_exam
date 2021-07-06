use dfn_candid::candid;
use dfn_core::over;
use candid::{candid_method, CandidType};

#[derive(CandidType)]
struct A {
    a: u16,
    b: u16,
}

#[derive(CandidType)]
struct B(u16, u16);

#[export_name = "canister_query greeting"]
fn greeting() {
    over(candid, |(name, age): (String, u16)| -> String {
        greeting_(name, age)
    })
}

#[candid_method(query, rename = "greeting")]
fn greeting_(name: String, age: u16) -> String {
    format!("Hello {}, you are {} years old", name, age)
}


#[export_name = "canister_query sum"]
fn combine() {
    over(candid, |(a, b): (u16, u16)| -> (u16, u16) {
        sum_(a, b)
    })
}

#[candid_method(query, rename = "sum")]
fn sum_(a: u16, b: u16) -> (u16, u16) {
    let x = a + b;
    let y = a - b;
    (x, y)
}


#[export_name = "canister_query sum2"]
fn combine2() {
    over(candid, |(a, b): (u16, u16)| -> u16 {
        sum2_(a, b)
    })
}

#[candid_method(query, rename = "sum2")]
fn sum2_(a: u16, b: u16) -> u16 {
    let x = a + b;
    x
}

#[export_name = "canister_query sum3"]
fn combine3() {
    over(candid, |(a, b): (u16, u16)| -> B {
        sum3_(a, b)
    })
}

#[candid_method(query, rename = "sum3")]
fn sum3_(a: u16, b: u16) -> B {
    let x = a + b;
    B(x, a)
}

#[export_name = "canister_query add"]
fn add() {
    over(candid, |(a, b): (u16, u16)| -> A {
        add_(a, b)
    })
}

#[candid_method(query, rename = "add")]
fn add_(a: u16, b: u16) -> A {
    A {
        a, b
    }
}

#[cfg(any(target_arch = "wasm32", test))]
fn main() {}

#[cfg(not(any(target_arch = "wasm32", test)))]
fn main() {
    candid::export_service!();
    std::print!("{}", __export_service());
}