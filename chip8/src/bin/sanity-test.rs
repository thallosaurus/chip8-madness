use core::str;
use std::cell::RefCell;

type HeaderBytes = [u8; 4];

#[derive(Debug)]
struct State {
    b: RefCell<HeaderBytes>
}

impl State {
    fn getTestTrait(self) -> impl TestTrait {
        self
    }
}

impl Default for State {
    fn default() -> Self {
        Self { b: RefCell::new([0x41, 0x41, 0x41, 0x41]) }
    }
}

fn main() {
    let s = State::default();

    let tt = s.getTestTrait();
    
    println!("{:?}", tt.getBytesAsString())
}

trait TestTrait {
    fn getBytesRef(&self) -> RefCell<HeaderBytes>;

    fn getBytesAsString(&self) -> String {
        let b = self.getBytesRef();
        //std::str::from_utf8(self.getBytesAsString())
        let v = Vec::from(*b.borrow());
        String::from_utf8(v).expect("error")
    }
}

impl TestTrait for State {
    fn getBytesRef(&self) -> RefCell<HeaderBytes> {
        self.b.clone()
    }
}