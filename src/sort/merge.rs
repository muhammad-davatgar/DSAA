use std::cmp::Ord;

pub fn merge_sort<T: Ord + Clone>(arg: &Vec<T>) -> Vec<T> {
    if arg.len() == 1 {
        return arg.clone();
    }
    let (arr1, arr2) = divide(arg);
    sort(merge_sort(&arr1), merge_sort(&arr2))
}

fn divide<T: Ord + Clone>(arg: &Vec<T>) -> (Vec<T>, Vec<T>) {
    let mid: usize = arg.len() / 2;

    (arg[0..mid].to_vec(), arg[mid..arg.len()].to_vec())
}
fn sort<T: Ord + Clone>(arr1: Vec<T>, arr2: Vec<T>) -> Vec<T> {
    let (mut a, mut b) = (0, 0);
    let mut res = Vec::new();
    for _ in 0..(arr1.len() + arr2.len()) {
        match (arr1.get(a), arr2.get(b)) {
            (Some(v1), Some(v2)) => {
                if v1 <= v2 {
                    res.push(v1.clone());
                    a += 1;
                } else {
                    res.push(v2.clone());
                    b += 1;
                }
            }
            // we could also use drain here but let's keep it simple
            (Some(v), None) => {
                res.push(v.clone());
                a += 1;
            }
            (None, Some(v)) => {
                res.push(v.clone());
                b += 1;
            }
            (None, None) => {
                panic!("it shouldn't get here");
            }
        }
    }

    return res;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn divide_works() {
        let tmp1 = vec![1, 2, 3, 4, 5];

        assert_eq!(divide(&tmp1), (vec![1, 2], vec![3, 4, 5]));

        let tmp2 = vec![1, 2, 3, 4, 5, 6];

        assert_eq!(divide(&tmp2), (vec![1, 2, 3], vec![4, 5, 6]));
    }

    #[test]
    fn sort_works() {
        let arr1 = vec![1, 3, 5, 7];

        let arr2 = vec![2, 4, 6, 8, 9];

        assert_eq!(sort(arr1, arr2), vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn merge_sort_works() {
        let arr = vec![4, 5, 6, 3, 2, 6, 0, 1];

        assert_eq!(merge_sort(&arr), vec![0, 1, 2, 3, 4, 5, 6, 6]);
    }
}
