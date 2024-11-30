use core::num;
use std::iter::Enumerate;

fn main() {
    let message = "Hello, dunia!";
    println!("{}", message);
}

#[test]
fn learn_variable() {
//    immutable
    let x = 5;
//    mutable
    let mut var1 = 21;
    println!("{}",var1);
    var1 = 5;
    println!("{}",var1);
    var1 = 6;
    println!("x: {}, var2: {}", x, var1);
}

#[test]
fn data_type() {
    let var1: i16 = 176;
    let var2 = 10.5;
    let var3: i8 = var1 as i8;
    println!("{}", var1);
    println!("{}", var2);
    println!("{}", var3);
}

#[test]
fn numeric() {
    let a = 100;
    let b = 20;
    let mut c = a * b;
    println!("{}", c);
    c = a / b;
    println!("{}", c);
    c = a + b;
    println!("{}", c);
    c = a - b;
    println!("{}", c);
    c = a % b;
    println!("{}", c);
    c += 10;
    println!("{}", c);
    c -= 10;
    println!("{}", c);
    c /= 10;
    println!("{}", c);
    c *= 10;
    println!("{}", c);
}

#[test]
fn boolean_type() {
    let a = true;
    let b = false;
    let mut c = a == b;
    println!("{}", c);
    c = !a == b;
    println!("{}", c);
    c = a && b;
    println!("{}", c);
    c = a || b;
    println!("{}", c);
}

#[test]
fn char_type() {
    let char1 = 'a';
    println!("{}", char1);
}

#[test]
fn tuple_type() {
    let mut datas = (10, 2.5, 'a');
    println!("{:?}", datas);
    let var1 = datas.0;
    println!("{}", var1);

    let (a, b, _) = datas;
    println!("{},{}", a, b);

    datas.0 = 27;
    println!("{:?},{}", datas, b);

    let tuple_kosong: () = ();
    let book = ("Title", "Description", "Price");

    println!("{:?},{:?}", tuple_kosong, book);
}

#[test]

fn array_type() {
    let mut arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", arr);
    println!("{}", arr[0]);

    arr[0] = 2;
    println!("{:?}", arr);

    let arr_len = arr.len();
    println!("{}", arr_len);

    let arr2: [[i32; 3]; 2] = [[1, 2, 3], [3, 2, 1]];
    println!("{}",arr2[0][1]);

    arr.iter().enumerate().for_each(|(index,&value)| {
        println!("Item ke {} : {}",index,value);
    });

    struct Book{
        title : String,
        desc : String,
        price : i64,
    }

    impl Book{
        fn new(title : &str, desc : &str, price : i64)->Self{
            Book {
                title : title.to_string(),
                desc : desc.to_string(),
                price : price
            }
        }

        fn display(&self, id : i64){
            println!("ID : {}, Title : {}, Desc : {}, Price : {}", id ,self.title, self.desc, self.price);
        }
    }
 
    let my_books : [Book;2] = [Book::new("Rust Programming", "Desc rust", 10000),Book::new("Rust Programming", "Desc rust", 10000)];
    my_books.iter().enumerate().for_each(|(index, item)|{
        item.display(index as i64);
    });

}



#[test]
fn constant_type(){
    const MAXIMUM:i32 = 2000;
    const MINIMUM:i32 = 0;
    println!("{},{}",MAXIMUM,MINIMUM);
}


#[test]
fn variable_scope(){
    let ori = 1;
    {
        println!("{}",ori);
        let tanjung = 2;
        println!("{}",tanjung);
    }
    // println!("{}",tanjung);
}



fn function_a(){
    let a = 10;
    let b = String::from("Orias");

    println!("{},{}", a, b);
}

fn function_b(){
    let a = 10;
    let b = String::from("Orias");

    println!("{},{}", a, b);
}
#[test]
fn stack_heap(){
    function_a();
    function_b();
}



#[test]

fn string_type(){
    let name = "   Orias tanjung   ";
    let trim = name.trim();
    println!("{}. {}", name, trim);


    let mut username = "orias";
    username="Budi"; // ini sebenarnya fix, usernamenya di buat variable baru, sehinnga 'orias' ga pernah hilang
    // melainkan dibuatin let username baru yg isinnya 'Budi'

    println!("{}", username);


    let mut name1 = String::from("Orias");
    name1.push_str(" Tanjung");
    println!("{}",name1);

    name1.replace("Orias", "Ate"); // ga akan berubah, karena dia replace harus ditampung ke var
    let newName1 = name1.replace("Orias", "Ate");
    println!("{}",newName1);

}


#[test]
fn ownership_rules(){
    let a= 20;
    println!("{}",a);
    {
        let b = 24;
        println!("{}",b);
    }
    // println!("{}",b); error karena ownership nya udah selesai setelah keluar dari scope b
    println!("{}",a);
}

#[test]

fn data_copy(){
    // hanya berlaku ke memory yg disimpan di stack
    let a = 20;
    let mut b = a; // ini artinya b itu copy value a, karena tiap var cmn boleh 1 owner
    println!("{}",a);
    println!("{}",b);
    b = 10;
    println!("{}",a); // nilai a ga akan berubah
    println!("{}",b);
}

#[test]
fn ownership_movement(){
    // berlaku pada heap memory, sehingga variable ownership lama ga akan bisa diakses lagi
    let name1 = String::from("orias");

    let name2 = name1;
    // println!("{}",name1); // error karena name1 sudah di borrow oleh name2 sehingga ga bisa diakses lagi
    println!("{}",name2);
    
    let name3 = name2.clone(); // ini dia nge copy value name2 ke name3, sehingga ownership nya tidak berubah
    println!("{}",name2);
    println!("{}",name3);

}


#[test]
fn clone(){
    // semua tipe data yg dissimpan di heap seperti String akan punya method .clone
    let name1 = String::from("Ori");
    let name2 = name1.clone();
    println!("{}",name1); // ga error karena udah di copy
    println!("{}",name2); // ga error karena udah di copy
}

#[test]
fn if_expression(){
    let val = 7;
    if (val >= 8) {
        println!("{}",val);
    }else if (val ==7) {
        println!("ini harusnya 7, val = {}",val);
    }else{
        println!("ga >= 8 atau == 7");
    }
}


#[test]
fn let_statement(){
    //cara manual
    let value = 9;
    let result :&str;

    if value >= 8 {
        result = "Good";
    }else if value < 8 && value >= 6 {
        result = "Mid";
    }else{
        result = "Bad";
    }

    println!("{}",result);

    // cara cepat

    let result1: &str =  if value >= 8 {
        "Good" // karena if else experssion di rust dia itu merupakan expression yang mengembalikan value
    }else if value < 8 && value >= 6 {
        "Mid"
    }else{
        "Bad" // ga boleh ada titik koma tapi
    };

    println!("{}",result1);
}


#[test]
fn loop_expression(){
    let mut counter = 0;
    loop{ // ini while loop sebenearnya
        counter+=1;
        if counter == 10{
            break;
        }else if counter %2 == 0{
            continue;
        }

        println!("Counter : {}", counter);
    }

    // bisa pakai let statement juga
    let mut counter1 = 0;
    let result = loop{
        counter1 += 1;
        if counter1 > 10{
            break counter1 *2; // return value nya inni
        }
    };

    println!("Result >> {}", result);
}


#[test]
fn loop_label(){
    let mut number = 1;
    'outer : loop{
        let mut i = 1;
        loop{
            if number > 10{
                break 'outer;
            }
            println!("{} x {} = {}", number, i, number * i);
            i+=1;
            if i >= 10{
                break;
            }
        }
        number +=1;
    }
}


#[test]
fn while_loop(){
    // sama kek loop cuman ada kondisi
    let mut num = 0;
    while true {
        num +=1;
        if num >=3{
            break;
        }
    }
    println!("num : {}",num);
}


#[test]
fn for_loop(){
    // sering pakai di kasus array iteration
    // contoh pkai while loop
    let arr = ['A','B','C','D','E'];
    let mut index = 0;
    while index < arr.len() {
        println!("array : {}, index : {}", arr[index], index);
        index+=1;
    }

    // pakai for loop
    for value in arr{
        println!("arr : {}", value);
    }
    // pakai index
    for (index,value) in arr.iter().enumerate(){
        println!("array : {}, index-i : {}", value, index);
    } 

    // bisa juga pakai range
    // syntax range = start_idx..end_idx
    let range_var = 0..5;
    println!("Start idx : {}", range_var.start);
    println!("End idx : {}", range_var.end);

    for i in range_var{
        println!("array : {}", arr[i]);
    }

    for i in 0..5{
        println!("array : {}", arr[i]);
    }

    // dengan range inclusive ( sesuai dengan nilai index, misal index akhir 4 ya sampai 4 indexnya, kalo range biasa dia length nya misal 0-5 berarti 0,1,2,3,4)
    // syntax range = start_idx..=end_idx
    let range_inclusive = 0..=4;
    println!("Start idx : {}", range_inclusive.start());
    println!("End idx : {}", range_inclusive.end());

    for i in range_inclusive{
        println!("array : {}", arr[i]);
    }

    for i in 0..=4{
        println!("array : {}", arr[i]);
    }

}

#[test]
fn learning_function(){
    fn greet(){
        println!("Helloow");
    }
    greet();
    greet();
    greet();

    // parameter dan argument sama aja di function, harus di kasih tipe datanya
    fn say_goodbye(username:&str, addition_message:&str){
        println!("Bye bye {} {}", username,addition_message);
    }

    say_goodbye("Orias", "See yaa!");
    say_goodbye("Oriiiiii", "");

    // dengan return value
    fn sum(num1:i32, num2:i32)->i32{
        return num1 + num2;
    }

    let sum = sum(5, 10);
    println!("sum of 5+10 = {}",sum);

    fn factorial(n:i32)-> i32{
        if n< 1{
            return 0;
        }
        let mut result= 1;
        for i in 1..=n{
            result*=i;
        }
       result // ini sebagai return value yg ga perlu ; lagi
    }

    let factorial_num=factorial(3);
    println!("factorial dari 3 = {}", factorial_num);


    fn factorial_recursive(n:i32)->i32{
        if n <=1 {
            return 1;
        }
        n *factorial_recursive(n-1) // ini udah di return
    }

    let factorial_rec = factorial_recursive(3);
    println!("factorial dari 3 = {}",factorial_rec);
}

// pakai module
mod new_main;
#[test]
fn module(){
    new_main::say_hallow();
}

// pakai subdirektori
mod utils;
#[test]
fn subdirektori(){
    utils::utils::midware();
    
}