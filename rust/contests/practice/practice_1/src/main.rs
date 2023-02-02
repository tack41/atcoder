use std::io;

fn main() {

    //1行目を読み込み
    let mut row1 = String::new();
    io::stdin().read_line(&mut row1).expect("Failed to read 1st row."); 
//    println!("row: {}", row);
    let a: i32 = row1.trim().parse::<i32>().unwrap();
//    println!("a: {}",a);

    //2行目を読み込み
    let mut row2 = String::new();
    io::stdin().read_line(&mut row2).expect("Failed to read 2nd row."); 
    let b_c: Vec<&str> = row2.trim().split(" ").collect();
//    println!("b_c[0]: {}, b_c[1]: {}", b_c[0], b_c[1]);
    let b: i32 = b_c[0].parse::<i32>().unwrap();
    let c: i32 = b_c[1].parse::<i32>().unwrap();
//    println!("b: {}, c: {}",b,c);

    //3行目を読み込み
    let mut row3 = String::new();
    io::stdin().read_line(&mut row3).expect("Failed to read 3rd row."); 
//    println!("row3: {}", row3);
    let s: &str = row3.trim();
//    println!("s: {}",s);

    println!("{} {}",(a+b+c), s);
} 