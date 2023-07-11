use std::{collections::btree_map::Values, io::stdout};

fn main() {
    let stdout = std::io::stdout();

    const NAME: usize = 20;

    println!("Hello, world!");

    let str = "I am Lantz";

    let a = 8;

    let message = String::new();

    println!("Hei! {str}");

    // 重影 shdowing
    let mut num = 12;
    num = 34;
    let mut num = 41;
    num = 32;
    // let num = '32'; // 报错

    println!("{0}", num); // 它将把之后的可变参数当作一个数组来访问，下标从 0 开始

    println!("{num}"); // num
    println!("{{}}"); // {}

    let arr = ["Lantz", "Fancy"];
    let arr_2 = [2; 5];

    // 报错
    // let SIZE = 20;
    // let arr_3 = [23; SIZE];

    const SIZE: usize = 20;

    let arr_3 = [23; SIZE];

    // 数组访问
    println!("{:?}", arr); // ["Lantz", "Fancy"]
    println!("{}", arr_2[0]);

    for index in 0..arr.len() {
        println!("index is {}", arr[index])
    }

    for val in arr.iter() {
        println!("value is {}", val);
    }

    //  报错: 数组arr_2不可变
    // arr_2[0] = 12;

    // let mut arr_3 = [3, 89, "Lantz"];

    println!("{}", arr.len());
}
