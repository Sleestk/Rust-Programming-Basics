use super::super::foo;
        
        pub fn print_foo() {
            foo::print();
        }

        pub fn print() {
            println!("a");
        }

        pub struct S {
            pub id: u32,
            name: String
        }

        pub fn build(id: u32) -> S {
            S {
                id,
                name: "".to_string()
            }
        }