use std::{cell::RefCell, borrow::Borrow};

use candid::{Deserialize, Encode, CandidType};
use ic_cdk::{export::serde::Serialize, api::{stable, self}};

// static WASM_PAGE_SIZE_IN_BYTES:usize = 64 *1024; //64KB
// static RESERVED_SPACE: usize = 8000;
// static mut THRESHOLD:u64 = 4294967296; //4G

#[derive(Serialize, Deserialize, Clone, CandidType)]
pub struct ComplexA {
    pub a: Vec<u8>,
}

impl ComplexA {
    fn new() -> Self {
        Self { 
            a:  [0 as u8].repeat(1024*256),
         }
    }
}

thread_local! {
    static INFO: RefCell<Vec<ComplexA>> = RefCell::new(Vec::new());
    static BYTE: RefCell<Vec<u8>> = RefCell::new(Vec::new());
}

#[ic_cdk_macros::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[ic_cdk_macros::update]
pub fn moreA () {
    INFO.with(|info|{
        for _ in 0..500 {
            let element = ComplexA::new();
            info.borrow_mut().push(element);
        }
    })
}

// pub fn get_size() -> usize {

// }

#[ic_cdk_macros::update]
pub fn put() -> usize {
    api::stable::stable_grow(8000);
    let  _info = INFO.with(|info|{
       info.borrow().clone()
    });
    let byte = Encode!(&_info).unwrap();    
    write_buffer(byte.clone());
    api::print(format!("{:?}",byte));
    byte.len()
}


pub fn write_buffer (value: Vec<u8>) {
    stable::stable_write(0, &value);
}


#[ic_cdk_macros::query]
pub fn read() -> usize{
    let mut res = vec![];
    let buff = read_buf();
    res.push(buff);
    res.len()
    
}

fn read_buf() -> Vec<u8>{
    let mut buf = [0].repeat(1024*256*500 as usize);
    stable::stable_read(0, &mut buf);
    buf.clone()
}
