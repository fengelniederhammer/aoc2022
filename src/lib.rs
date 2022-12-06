extern crate core;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

#[macro_export]
macro_rules! tests {
    ($b:expr) => {
        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn test_ex1() {
                assert_eq!(
                    $b,
                    ex1(
                        std::fs::read_to_string(format!("src/{}/testInput", crate::day!()))
                            .unwrap()
                    )
                );
                println!(
                    "{:?}",
                    ex1(std::fs::read_to_string(format!("src/{}/input", crate::day!())).unwrap())
                );
            }
        }
    };
    ($b:expr,$c:expr) => {
        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn test_ex1() {
                assert_eq!(
                    $b,
                    ex1(
                        std::fs::read_to_string(format!("src/{}/testInput", crate::day!()))
                            .unwrap()
                    )
                );
                println!(
                    "{:?}",
                    ex1(std::fs::read_to_string(format!("src/{}/input", crate::day!())).unwrap())
                );
            }

            #[test]
            fn test_ex2() {
                assert_eq!(
                    $c,
                    ex2(
                        std::fs::read_to_string(format!("src/{}/testInput", crate::day!()))
                            .unwrap()
                    )
                );
                println!(
                    "{:?}",
                    ex2(std::fs::read_to_string(format!("src/{}/input", crate::day!())).unwrap())
                );
            }
        }
    };
}

#[macro_export]
macro_rules! day {
    () => {
        module_path!().split("::").skip(1).next().unwrap()
    };
}
