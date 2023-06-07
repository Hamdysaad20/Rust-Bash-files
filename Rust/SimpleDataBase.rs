

struct User {
    id: Option<u32>,
    username: String,
    email: String,
    password: String,
}

impl User {
    fn new(username: &str, email: &str, password: &str) -> User {
        use std::sync::atomic::{AtomicU32, Ordering};

        static ID: AtomicU32 = AtomicU32::new(0);

        User {
            username: username.to_string(),
            email: email.to_string(),
            password: password.to_string(),
            id: Option::from(ID.fetch_add(1, Ordering::SeqCst)),
        }
    }

    fn summarize(&self) -> String {
        format!("Username: {}\nEmail: {}\nPassword: {}", self.username, self.email, self.password)
    }
}
//database class for user data
struct Database {
    users: Vec<User>,
}

impl Database {
    fn createID(&self) -> i32 {
        return self.users.len() as i32 + 1;
    }
    fn new(users: Vec<User>) -> Database {
        Database { users }
    }

    fn add_user(&mut self, user: User) {
        println!("User {} added", &user.username);
        self.users.push(user);
    }
    fn add_users(&mut self, users: Vec<User>) {
        for user in users {
            self.add_user(user);
        }
    }

    fn print_users(&self) {
        for user in &self.users {
            print!("users number {}\n", self.users.len());
            print!("user id {}\n", user.id.unwrap());
            println!("{}", user.summarize());
            print!("-------------------\n");
        }
    }

    fn remove_user(&mut self, user: &User) {
        let mut index = None;
        for (i, u) in self.users.iter().enumerate() {
            if u.username == user.username {
                index = Some(i);
                break;
            }
        }
        match index {
            Some(i) => {
                self.users.remove(i);
                println!("User {} removed", user.username);
            }
            None => println!("User {} not found", user.username),
        }
    }
}

fn main() {

    let mut _database = Database::new( Vec::new());

    let user1 = User::new("@hamdy20", "mail@mail.com","password");
// do it multi 

    _database.add_users( vec![user1, user2, user3, user4, user5, user6, user7, user8]);

    _database.print_users();


    }



