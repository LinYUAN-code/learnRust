extern crate core;
use core::slice;
pub mod learnMacro;
// dereferencing a raw pointer
// raw pointer 就像c/c++ 里面的指针一样---powerful
fn case1() {
    let mut num = 10;
    // We’ve created raw pointers by using as to cast an immutable and a mutable reference into their corresponding raw pointer types
    // 我们可以在safe的情况下创建一个raw pointer 但是 我们不能dereferenceing
    // 因为问题只会发生在访问指针的时候
    // 我觉得mut 和 const 区分真的很不错 比c系 的 const指针好多了
    // raw pointer 我们可以让 mut 指针 和 immut指针共存 可能导致 race 需要注意
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    let address = 0x012345usize;
    let r = address as *const i32;
    unsafe {
        println!("derefe raw pointer r1 {}", *r1);
        println!("derefe raw pointer r2 {}", *r2);
    }
}

unsafe fn dangerous() {}

fn case2() {
    unsafe {
        dangerous();
    }
}

// 自己实现一个split_at_mut
fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

fn case3() {
    let mut arr = vec![1, 2, 3, 4, 5, 6, 7];
    let r = &mut arr[..];

    let (a, b) = r.split_at_mut(3);

    println!("{:?}", a);
    println!("{:?}", b);
}

// static/const 关键字在rust 中可以用来声明全局变量 --- rust全局变量没有和c++一样有 static 和 非static的区别 作用域可以用 pub 来声明
// Static variables can only store references with the 'static lifetime
// mut 全局变量是不安全的 访问和修改都需要unsafe
static mut COUNTER: u32 = 0;
fn case4() {
    unsafe {
        COUNTER += 1;
    }
    unsafe {
        println!("{}", COUNTER);
    }
}

trait Animal {
    fn baby_name();
}

struct Dog;

impl Dog {
    fn baby_name() {
        println!("dog");
    }
}
impl Animal for Dog {
    fn baby_name() {
        println!("Animal");
    }
}
fn case5() {
    Dog::baby_name();
    <Dog as Animal>::baby_name();
}

fn main() {
    // case1();
    // case2();
    // case3();
    case5();
    learnMacro::case1();
}
