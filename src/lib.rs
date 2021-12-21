#![no_std]

use codec::{Decode};
use gstd::{msg, prelude::*};
use scale_info::TypeInfo;

#[derive(TypeInfo, Decode)]
pub enum Action {
    Increment,
    ViewCounter,
}

gstd::metadata! {
    title: "Counter",
    handle:
        input: Action,
        output: u64,
}

static mut COUNTER: u64 = 0;

#[no_mangle]
pub unsafe extern "C" fn handle() {
    let action: Action = msg::load().unwrap();

    match action {
        Action::Increment => {
            COUNTER+=1;
        }
        Action::ViewCounter => {
            msg::reply(&COUNTER, 12_000_000, 0);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn init() {}