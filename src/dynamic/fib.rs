use std::{cell::RefCell, collections::HashMap};

pub fn dynamic_fibonacci(n: u128, memo: &RefCell<HashMap<u128, u128>>) -> u128 {
    if n <= 2 {
        return 1;
    }

    if let Some(v) = memo.borrow().get(&n) {
        return *v;
    }

    let res = dynamic_fibonacci(n - 1, &memo) + dynamic_fibonacci(n - 2, &memo);
    memo.borrow_mut().insert(n, res);
    return res;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn dynamic_fibonacci_works() {
        let memo = RefCell::new(HashMap::new());

        assert_eq!(12586269025, dynamic_fibonacci(50, &memo));
    }
}

// fn dynamic_fibonacci(n: u128) -> u128 {
//     if n <= 2 {
//         return 1;
//     } else {
//         return dynamic_fibonacci(n - 1) + dynamic_fibonacci(n - 2);
//     }
// }
