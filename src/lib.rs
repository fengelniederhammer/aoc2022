extern crate core;

mod day1;
mod day2;
mod day3;
mod day4;

#[macro_export]
macro_rules! tests {
    ($b:expr) => {
        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn test_ex1() {
                assert_eq!($b, ex1(read_to_string(format!("src/{}/testInput", crate::day!())).unwrap()));
            }

            #[test]
            fn answer1() {
                println!("{}", ex1(read_to_string(format!("src/{}/input", crate::day!())).unwrap()));
            }
        }
    };
    ($b:expr,$c:expr) => {
        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn test_ex1() {
                assert_eq!($b, ex1(read_to_string(format!("src/{}/testInput", crate::day!())).unwrap()));
            }

            #[test]
            fn answer1() {
                println!("{}", ex1(read_to_string(format!("src/{}/input", crate::day!())).unwrap()));
            }

            #[test]
            fn test_ex2() {
                assert_eq!($c, ex2(read_to_string(format!("src/{}/testInput", crate::day!())).unwrap()));
            }

            #[test]
            fn answer2() {
                println!("{}", ex2(read_to_string(format!("src/{}/input", crate::day!())).unwrap()));
            }
        }
    }
}

#[macro_export]
macro_rules! day {
    () => {
        module_path!().split("::").skip(1).next().unwrap()
    }
}
