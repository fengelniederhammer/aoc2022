extern crate core;

mod day1;
mod day2;
mod day3;

#[macro_export]
macro_rules! tests{
    ($a:ident,$b:expr,$c:expr)=>{
        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn test_ex1() {
                assert_eq!($b, ex1(read_to_string(format!("src/{}/testInput", stringify!($a))).unwrap()));
            }

            #[test]
            fn answer1() {
                println!("{}", ex1(read_to_string(format!("src/{}/input", stringify!($a))).unwrap()));
            }

            #[test]
            fn test_ex2() {
                assert_eq!($c, ex2(read_to_string(format!("src/{}/testInput", stringify!($a))).unwrap()));
            }

            #[test]
            fn answer2() {
                println!("{}", ex2(read_to_string(format!("src/{}/input", stringify!($a))).unwrap()));
            }
        }
    }
}