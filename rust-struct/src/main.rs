// make struct
struct Animal {
    name: String,
    age: i32,
}
impl Animal {
    fn say_name(&self) {
        // pass : stack get: heap
        println!("Hello make is: {}", self.name)
    }
    fn change_age(&mut self) {
        // this is double check form rust , if need change the door of owner must open
        self.age = 30;
    }
}
trait Ras {
    fn yellow(&self);
}
impl Ras for Animal {
    fn yellow(&self) {
        println!("hello warna saya adalah kuning");
    }
}
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    login_count: i32,
}
fn create_user(active: bool, username: String) -> User {
    User {
        active,
        username,
        login_count: 1,
    }
}
fn main() {
    let user_1 = create_user(true, String::from("Crocodile"));
    println!("user now is : {:?}", user_1);
    let user_2 = User {
        active: true,
        username: String::from("Bob"),
        login_count: 1,
    };
    println!("user now is : {:?}", user_2);
    let user_3 = User {
        active: false,
        ..user_2
    };
    println!("user now is : {:?}", user_3);

    let mut macan_asia: Animal = Animal {
        name: String::from("Macan-asia"),
        age: 12,
    };

    macan_asia.say_name();
    println!("the animal name is : {}", macan_asia.name);
    macan_asia.change_age();
    println!("the animal age is : {}", macan_asia.age);
    macan_asia.yellow();
}
