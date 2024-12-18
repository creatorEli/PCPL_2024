// взял вторую лабу и выполню то же задание на Rust.
use std::fs;
use std::fs::File;
use std::io::prelude::*;

use std::time::{Duration, Instant};

fn quick_sort<T: Ord>(mut arr: Vec<T>) -> Vec<T> { // шаблонная функция, работает с любыми сравниваемыми типамиЫ
    if arr.len() <= 1 {
        return arr;
    }

    let pivot = arr.remove(0);
    let mut left = vec![];
    let mut right = vec![];

    for item in arr {
        if item <= pivot {
            left.push(item);
        } else {
            right.push(item);
        }
    }

    let mut sorted_left = quick_sort(left);
    let mut sorted_right = quick_sort(right);

    sorted_left.push(pivot);
    sorted_left.append(&mut sorted_right);

    sorted_left
}


fn main() {

    let start = Instant::now();

    let file_path = "rand.txt";

    let contents = fs::read_to_string(file_path)
        .expect("Не можем прочитать файл!");

    //println!("With text:\n{contents}");

    let nums  = contents.lines();

    let mut count = 0;

    // for num in nums{
    //     //println!("{}", num);
    //     count += 1;
    // }



    let mut numsVec: Vec<i32> = Vec::new();
    //let mut arr: [i32; 9];

    //let mut i = 0;
    for num in nums{
        let el = num.parse::<i32>().unwrap();
        numsVec.push(el);
        //println!("{}", el);
        count+=1;
        //arr[i] = el;
        //i+=1;
    }

    //println!("completed creating Vector!");

    let start_sort = Instant::now();

    let res = quick_sort(numsVec);

    let dur_sort = start_sort.elapsed();

    //println!("completed sorting Vector!");


    let mut result :String = "".to_string();

    //for el in numsVec{
    for el in res{
        //result += el.to_string();
        result.push_str(&el.to_string());
        result.push('\n');
    }

    //println!("{result}");

    let mut file = fs::File::create("SortedRust.txt").expect("Ошибка чтения файла!");
    //file.write_all(result.as_bytes());
    write!(file, "{result}").expect("Ошибка записи файла!");

    let end = start.elapsed();

    println!("Массив отсортирован. Время сортировки: {:?}ms;", dur_sort);
    println!("Время работы всей программы: {:?}ms;", end); 

}