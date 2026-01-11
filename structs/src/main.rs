#![allow(unused)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
    // empresa: $str; // (COMPILER ERROR) <- expected a lifetime parameter
}

impl User {
    fn set_email(&mut self, email: String) -> () {
        self.email = email;
    }
    fn get_email(&self) -> String {
        self.email.clone()
    }
    fn delete_x(self) -> () {
        // delete user, because it is moved
    }
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}



fn main() {
    

    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    user1.delete_x();
    // println!("User1: {}", user1.get_email()); (COMPILER ERROR) <- user1 is moved at delete_x()


    
    let user2 = build_user(
        String::from("pinto@outlook.com"),
        String::from("pinto")
    );
    // user2.username = String::from("ponto"); // (COMPILER ERROR) <- user2 is immutable



    let _user3 = User {
        email: String::from("ponto@outlook.com"),
        ..user2 // preenche the rest of the fields with user2 (update Syntax)
        // doing this will delete user2 username
    };
    //let _x = user2.username;   // (COMPILER ERROR) <- user2 name is moved
    let _x = user2.email;        // (OK) e-mai was not used
    let _x = user2.active;       // (OK) active is a copy() type
    let _x = user2.sign_in_count;// (OK) is a copy() type







}
