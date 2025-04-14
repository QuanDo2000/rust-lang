//* Listing 5-1
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
//* 5-1

//* Listing 5-4
fn build_user(email: String, username: String) -> User {
    // User {
    //     active: true,
    //     username: username,
    //     email: email,
    //     sign_in_count: 1,
    // }

    //* Listing 5-5
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
    //* 5-5
}
//* 5-4

fn main() {
    //* Listing 5-2
    // let user1 = User {
    //     active: true,
    //     username: String::from("someusername123"),
    //     email: String::from("someone@example.com"),
    //     sign_in_count: 1,
    // };
    //* 5-2

    //* Listing 5-3
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
    //* 5-3

    //* Listing 5-6
    // let user2 = User {
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("another@example.com"),
    //     sign_in_count: user1.sign_in_count,
    // };
    //* 5-6

    //* Listing 5-7
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    //* 5-7
}
