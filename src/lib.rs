mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

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
            }

            fn print_ex1() {
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
            }

            #[test]
            fn print_ex1() {
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
            }

            #[test]
            fn print_ex2() {
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
