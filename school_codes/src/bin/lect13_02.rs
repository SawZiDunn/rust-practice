fn f() {
    print!("f ");
    g();
    m::f();
    m::m::f();
}

fn g() { print!("g "); }

mod m {
    pub fn f() {
        print!("1.f ");
        g();
        m::f();
        super::g();
    }

    fn g() { print!("1.g "); }
    
    pub mod m {
        pub fn f() {
            print!("2.f ");
            g();
            super::g();
            super::super::g();
            crate::g();
        }
        fn g() { print!("2.g "); }
    }
}

fn main() {
    f();
}