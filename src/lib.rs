// struct TypeA {
//     x: i32,
// }

// struct TypeB {
//     x: i16,
//     y: i16,
//     z: i32,
// }

// trait Sum {
//     fn sum(&self) -> i32;
// }

// impl Sum for TypeA {
//     fn sum(&self) -> i32 {
//         self.x
//     }
// }

// impl Sum for TypeB {
//     fn sum(&self) -> i32 {
//         self.x as i32 + self.y as i32 + self.z
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let type_a = TypeA { x: 10 };
//         let type_b = TypeB {
//             x: 10,
//             y: 10,
//             z: 10,
//         };
//         assert_eq!(10, type_a.sum());
//         assert_eq!(30, type_b.sum());
//     }

//     #[test]
//     fn test_size_of() {
//         let size_of_type_a = size_of::<TypeA>();
//         let size_of_type_b = size_of::<TypeB>();
//         assert_eq!(size_of_type_a, 4);
//         assert_eq!(size_of_type_b, 8);
//     }

//     #[test]
//     fn test_dyn() {
//         let type_a = TypeA { x: 10 };
//         let type_b = TypeB {
//             x: 10,
//             y: 10,
//             z: 10,
//         };
//         let vec: Vec<Box<dyn Sum>> = vec![Box::new(type_a), Box::new(type_b)];
//         let all_sum = vec.iter().map(|item| item.sum()).sum();
//         assert_eq!(40, all_sum);
//         let a = &vec[0];
//         assert_eq!(10, a.sum());
//     }
// }
