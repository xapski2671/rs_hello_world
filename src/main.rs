struct Bird {
    name: String,
    attack: u64,
}

// to add methods to struct Bird
impl Bird {
    fn print_name(&self) {
        println!("{}", self.name);
    }
}

impl Animal for Bird {
    fn can_fly(&self) -> bool {
        true
    }
}

// rust does not support inheritance, it only supports interfaces(trait)
trait Animal {
    fn can_fly(&self) -> bool;
    fn is_animal(&self) -> bool {
        true
    }
}

// enums
#[derive(Debug)]
enum MyEnum {
    A,
    B(i32),
    C { x: i32, y: i32 },
}

fn main() {
    println!("Hello, world!");

    let mut x = 45;
    println!("The value of x is {}", x);

    x = 75;
    println!("The value of x is {}", x);

    // match statements
    let u = 5;
    match u {
        0 => println!("0"),
        1 | 2 => println!("1,2"),
        3..=4 => println!("3,4"), // =4 means 4 is inclusive in the range 3..4
        _ => println!("default"),
    };

    let bird = Bird {
        name: String::from("Mangal Drake"),
        attack: 5,
    };

    bird.print_name();
    println!("Attack value: {}", bird.attack);
    println!("{} {}", bird.can_fly(), bird.is_animal());

    let a: MyEnum = MyEnum::A;
    let b: MyEnum = MyEnum::B(5);
    let c: MyEnum = MyEnum::C { x: 4, y: 3 };

    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", c);

    if let MyEnum::B(val) = b {
        println!("Enum B is {}", val);
    }

    if let MyEnum::C { x, y } = c {
        println!("Enum C is {} {}", x, y);
    }

    // vectors
    let mut vec: Vec<i64> = vec![1, 2, 3, 4];
    vec.len();
    vec[0];
    vec.push(6);
    vec.remove(2);
}
