// use std::vec;
//
// fn find_largest(nums: &[i32]) -> i32 {
//     let mut largest = nums[0];
//
//     for &num in nums {
//         if num > largest {
//             largest = num;
//         }
//     }
//
//     largest
// }
//
// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];
//     let largest = find_largest(&number_list);
//     println!("The largest number is {largest}");
//     assert_eq!(largest, 100);
// }

// fn largest(list: &[i32]) -> i32 {
//     let mut largest = list[0];
//
//     for &item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//
//     largest
// }
//
// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];
//
//     let result = largest(&number_list);
//     println!("The largest number is {result}");
//     assert_eq!(result, 100);
//
//     let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
//
//     let result = largest(&number_list);
//     println!("The largest number is {result}");
//     assert_eq!(result, 6000);
// }

// 这段代码没法通过编译，因为这个函数没有实现: std::cmp::PartialOrd
// fn largest<T>(list: &[T]) -> T {
//     let mut largest = list[0];
//
//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }
//
//     largest
// }
//
// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];
//
//     let result = largest(&number_list);
//     println!("The largest number is {}", result);
//
//     let char_list = vec!['y', 'm', 'a', 'q'];
//
//     let result = largest(&char_list);
//     println!("The largest char is {}", result);
// }

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}
