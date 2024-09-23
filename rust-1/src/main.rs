// use std::fs;
// }
// struct User{
//     name : String,
//     price : i32,
//     role : String
// impl User{
    // }
//     fn give_price(&self)->i32{
//         return self.price;
//     }


// enum Directions {
//     North,
//     South,
//     East,
//     West
// }

// enum Shapes {
//     Circle(f64),
//     Rectangle(f64,f64),
//     Square(f64)
// }


// struct nums <T>{
//     x : T,
//     y : T
// }



fn main() {
    // println!("Hello, world!");
    // let x : i32 = -5;
    // let y : u32 = 100;
    // let z : f32 = 100.001;

    // println!("{} , {} , {} " , x , y, z);
    // println!("y : {}" , y);
    // println!("z : {}" , z);



    // let is_male : bool = true;
    // let is_18plus : bool = true;

    // if is_male{
    //     println!("You are a male")
    // }
    // else{
    //     println!("you are not a male")
    // }
    // if is_male && is_18plus {
    //     println!("You are a legal male")
    // }
    // else {
    //     println!("you are not legal")
    // }

    // let str : String = String::from("Hello World");
    // println!("{}" , str);

    // let str_at_index = str.chars().nth(10);
    // println!("{}",str_at_index.unwrap())  // will throw an error if i am trying to access a index that doesn't exist


    // better approach to handle string index
    // match str_at_index {
    //     Some(c) => println!("{}" , c),
    //     None => println!("Index is out of range"),
    // }

    // for i in 0..100{
    //     println!("{}",i);
    // }

    // Getting only a part of a string and heap storage of string.

    // let selected_sentence : String = String::from("name is Ayush");
    // println!("{}",selected_sentence);
    // println!("Length : {}, Capacity : {},Pointer : {:p}" , selected_sentence.len(),selected_sentence.capacity(),selected_sentence.as_ptr());
    // let word = get_first_str(selected_sentence);
    // println!("{}" , word);
    // println!("Length : {}, Capacity : {},Pointer : {:p}" , word.len(),word.capacity(),word.as_ptr());

    // OwnerShip 

    // let s1 = String::from("Muskan");
    // println!("s1 is boyfriend of : {}",s1);
    // let s2 = s1;
    // println!("s2 is boyfriend of : {}",s2);
    // println!("{}",s1);  // As s1 is not the owner of Muskan anymore,

    // Mutable and Immutable Borrowers
    // let mut s1 = String::from("Cristiano");
    // let _s3 = &mut s1;
    // let s2 = &s1;
    // immutable_borrower(&s1);
    // println!("{}",_s3);
    // println!("{}",s2);
    // println!("{}",s3);
    // println!("{}",s2);

    // Struct in Rust
    // let user = User{
    //     _name : String::from("Ayush"),
    //     price : 32,
    //     _role : String::from("Owner")
    // };
    // // println!("name is {} role is {} price is {}",user.name,user.role,user.price)
    // println!("{}",user.give_price())

    // Enums 
    // let direct = Directions::North;
    // let circle = Shapes::Circle(7.00);
    // let circle_area = calculate_area(circle);
    // println!("{}",circle_area);
    // println!(shape)

    //Generic
    // let a = nums{x : 5 , y : 7};
    // let b = nums{x : 7.0 , y : 7.7};

    // Error Handling
    // let res = fs::read_to_string("example.txt");
    // match res {
    //     Ok(content)=> println!("file content : {}",content),
    //     Err(content) => println!("Error : {}",content)
    // }

    // println!("hello error was handeled")

    // let path = String::from("example.txt");
    // // handle_file_unsafe(path);
    // let res = handle_file_safe(path);
    // match res {
    //     Ok(content) => println!("{}",content),
    //     Err(content) => println!("Error while reading : {}",content),
    // }

    //  Option enum
    // let str = String::from("Welcome to my drkside");
    // let res = find_index_a(str);
    // match res{
    //     Some(i) => println!("index of a in string is {}",i),
    //     None => println!("a not found in string")
    // }



}

// fn get_first_str(selected_string : String) -> String{
//     let mut ans : String = String::from("");
//     for char in selected_string.chars(){
//         if char == ' '{
//             break;
//         }
//         ans.push(char);
//     }
//     return ans;
// }

// fn immutable_borrower(word : &String){
//     println!("Coming Straight to you from function : {}",word);
// }

// fn calculate_area(shape : Shapes) -> f64 {
//     let ans = match shape{
//         Shapes::Circle(radius) => 3.14 * radius * radius,
//         Shapes::Rectangle(length,breadth) => length*breadth,
//         Shapes::Square(side) => side*side,
//     };
//     return ans;
// }

// fn handle_file_unsafe(path : String) -> String{
//     let res = fs::read_to_string(path);
//     return res.unwrap();
// }



// fn handle_file_safe(path : String) -> Result<String,String>{
//     let res = fs::read_to_string(path);
//     match res{
//         Ok(content) => return Ok(content),
//         Err(content) => return Err(content.to_string())
//     }
// }

// fn find_index_a(str : String) -> Option<usize>{
//     for (index,char) in str.chars().enumerate(){
//         if char == 'a'{
//             return Some(index);
//         }
//     }
//     return None;
// }