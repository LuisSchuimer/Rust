#[warn(dead_code)]

fn main() {
    //test();

    //let num: i32 = 100;
    //println!("{num} C° = {} fahrenheit", convert_to_fa(num));
    //println!("Hello World");
    //loop_test();
    test();
}

fn convert_to_fa(x: i32) -> i32 {
    (x * 9 / 5) + 32
}

fn loop_test() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn test() {
    //let x: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    //let y: [i32; 10] = [2, 3, 1, 9, 3, 2, 4, 1, 3, 4];
    //let z: [i32; 10] = [1, 4, 3, 4, 2, 6, 7, 8, 9, 9];
    //let a: [i32; 10] = [10, 19, 35, 100, 230, 290, 310, 331, 523, 1000];
    let mut b: [i32; 10] = [1, 5, 3, 4, 5, 3, 7, 2386, 9, 192];

    let reversed = reverse(&mut b);
    println!("{:?}", reversed);
    println!("{:?}", b);
    print!("{}", is_reversed(&reversed, &b));
    //let list_data: (i32, i32, f32) = get_list_data(b);
    //println!("{:?}", list_data);
    //let sorted = sort(b);
    //println!("Before: {:?}", b);
    //println!("After: {:?}", sorted);
}

fn reverse(x: &[i32; 10]) -> [i32; 10] {
    let mut reversed: [i32; 10] = [0; 10];
    for n in 0..x.len() {
        let item: i32 = x[n];
        reversed[(x.len() -1) - n] = item;
    }
    reversed
}

fn is_reversed(reversed: &[i32; 10], original: &[i32; 10]) -> bool {
    for n in 0..reversed.len() {
        if original[(reversed.len() -1) - n] != reversed[n] { return false; }
    }
    true
}
fn get_list_data(x: [i32; 10]) -> (i32, i32, f32) {
    let (mut smallest, mut biggest, mut average) = (i32::MAX, i32::MIN, 0_f32);
    for n in x {
        if n < smallest {smallest = n;}
        if n > biggest {biggest = n;}
        average += n as f32
    }
    (smallest, biggest, average/10_f32)
}

fn is_sorted(x: [i32; 10]) -> bool {
    let mut count: i32 = i32::MIN; //0

    for n in x {
        if n <= count { return false; } else { count = n; }
    }
    true
}
fn sort(x: [i32; 10]) -> [i32; 10] {
    let mut to_sort: [i32; 10] = x;
    let array_length = to_sort.len();
    for _ in 0..array_length {
        for value in 1..array_length-2 {
            if to_sort[value-1] > to_sort[value] {
                let larger_value: i32 = to_sort[value-1];
                to_sort[value-1] = to_sort[value];
                to_sort[value] = larger_value;
            }
        }
    }
    to_sort
}