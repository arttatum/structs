fn main() {
    // We can create an instance of a struct in several ways
    
    //A)
    let user = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    //B)
    let user1 = build_user(String::from("anEmail@examples.com"), String::from("Will"));
    
    println!("user1 email: {}", user1.email);

    //C)
    let mut user2 = build_user_shorthand(String::from("someones@examples.com"), String::from("Mx User"));
    
    println!("user2 email before mutation: {}", user2.email);
    
    user2.email = String::from("anotheremail@example.com");
    
    println!("user2 email after mutation: {}", user2.email);

    //D) Here we take any values that aren't specified below, from user2.
    
    let user3 = User {
        email: String::from("user3@example.com"),
        ..user2
    };

    println!("user3 email: {}\nuser3 username: {}", user3.email, user3.username);
    
    // Since the String field 'username' does not implement Copy, user2.username is 
    // 'moved' into user3.username, user2.username is no longer accessible.

    println!("user2 email: {}\n", user2.email); // this is fine
    //println!("user2 username: {}\n", user2.username); // this is not


}

struct User {
    active: bool,
    username: String, //String is an owned type... we can use references but will cover that in the 'lifetimes' project
    email: String,
    sign_in_count: u64,
}

// These are tuple structs, a method taking a Colour struct as a parameter will not accept a Point.
// Otherwise, they behave like tuples (can be destructured, values accessed by index)
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// This is a 'unit like' struct, it has no fields! It behaves like the unit type, () 
// ...more on this in the Traits project.
struct AlwaysEqual;

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

fn build_user_shorthand(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}