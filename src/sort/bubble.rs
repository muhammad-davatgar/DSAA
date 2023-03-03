pub fn bubble_sort<T: Ord>(arg: &mut Vec<T>) {
    for i in 0..arg.len() {
        for j in i..(arg.len()) {
            if arg[j] < arg[i] {
                arg.swap(i, j);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use std::vec;

    use super::*;

    #[test]
    fn bubble_sort_works() {
        let mut tmp = vec![1, 4, 6, 3, 5, 9];
        bubble_sort(&mut tmp);
        assert_eq!(tmp, vec![1, 3, 4, 5, 6, 9]);
    }
}
