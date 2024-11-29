use std::fmt;

// At Jack's question - it's also possible to extend a trait - inheritance style from OOP (C++, Python, Java etc.)
trait MyTrait: fmt::Debug {
    fn my_func(&self);
}

// Define a struct
struct Cat {
    name: String,
    age: i32,
}

// Implement a custom Debug for our Cat structure - note the argument and return types
// Should work for the structure and (as debug inherited generically not specifically, individual members)
impl fmt::Debug for Cat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cat {{ name: {}, value: {} }}", self.name, self.age)
    }
}

// Implement MyTrait for MyStruct
impl MyTrait for Cat {
    fn my_func(&self) {
        println!("Hello!");
    }
}

fn main() {
    let my_cat = Cat {
        name: String::from("Frank"),
        age: 19,
    };
    
    // Calling the method from custom MyTrait
    my_cat.my_func();

    // The struct is also automatically Debug since it implements Debug
    println!("{:?}", my_cat); 
    println!("{:?}", my_cat.name);
}