use std::{cmp::Ord, fmt::Display};

pub fn insertion_sort<T: Ord + Display>(input: &mut [T]) {
    for i in 1..input.len() {
        loop {
            if i != 0 && input[i] < input[i - 1] {
                input.swap(i, i - 1);
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insertion_sort_works() {
        let mut result = [4, 2, 3];
        insertion_sort(&mut result);
        assert_eq!(result, [2, 3, 4]);
    }
}
