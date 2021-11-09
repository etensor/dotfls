use std::io;


struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// TUPLES:

struct Color(i32,i32,i32);
struct Point(i32,i32,i32);


struct Rect{
    width: u32,
    heigth: u32,
}

impl Rect {
    fn area(&self) -> u32 {
        self.width * self.heigth
    }

    
}




fn main() {
    println!("Hello, world!");

    let green = Color(0,255,0);

    loop {
        println!("input guess: (better if number) ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");
    
        let numero: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Hello! your guess: {}",guess);

        if numero == 5{
            println!("good!");
            break;
        }
        
    }

    get_rusting(1,5);

    expressionsinrust();

    let numero_f = five();
    println!("{} five",numero_f);

    let arr = [1,2,3,4,5,6,7,8];


    for elem in arr.iter(){
        print!("{} | ",elem);
    }

    let mut s = String::from("EPA");

    s.push_str(", sisas rust");

    println!("{}",s);

    // drop === free === delete

    let word = first_word(&s);
    println!("{} <== ",word);

    let mut user1 = User {
        email: String::from("epa@correo.com"),
        username: String::from("username"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("postura_terrible@quegonorrea.com");

    let user2 : User = build_user(String::from("User"),String::from("Pass"));


    let user3 : User = User{
        email : String::from("another3@exam.ple"),
        username : String::from("user333"),
        ..user2 // ->> hereda datos.
    };

    println!("{}",user3.username);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,      // valid because arg has the exact same name
        username,
        active: true,
        sign_in_count: 1,
    }
}



fn get_rusting(x: i32, y: i32){
    println!("{} = x , {} = y | ",x,y);
}

fn expressionsinrust(){
    let y = {
        let x = 7;
        x + 1              // Expression ->> no ; | statement -> ;
    };

    println!("Value y: {}",y);
}

fn five() -> i32 {5}


fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}












