fn find_max<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut ans = list[0];
    // you are using pattern matching to explicitly destructure the reference
    // 想想为什么加上
    // ans =  &T --> typeof ans = &T
    // &ans  = &T --> typeof ans = T
    for &item in list {
        if item > ans {
            ans = item
        }
    }
    ans
}

fn main() {
    let arr1 = vec![1, 2, 3, 4, 5];
    println!("max {}", find_max(&arr1));
    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = find_max(&char_list);
    println!("The largest char is {}", result);
}
