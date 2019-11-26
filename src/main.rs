use algorithms_and_data_structures::binary_search::binary_search;

fn main() {
    println!("Hello, algorithms student!");

    let arr: [isize; 5] = [-1, 0, 1, 2, 3];
    let result = binary_search(&arr[..], -2);
    println!("{:?}", result);
    let result = binary_search(&arr[..], -1);
    println!("{:?}", result);
    let result = binary_search(&arr[..], 0);
    println!("{:?}", result);
    let result = binary_search(&arr[..], 1);
    println!("{:?}", result);
    let result = binary_search(&arr[..], 2);
    println!("{:?}", result);
    let result = binary_search(&arr[..], 3);
    println!("{:?}", result);
    let result = binary_search(&arr[..], 4);
    println!("{:?}", result);
}
