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