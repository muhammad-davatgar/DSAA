use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub fn dynamic_fibonacci(n: u128, memo: Rc<RefCell<HashMap<u128, u128>>>) -> u128 {
    if n <= 2 {
        return 1;
    }

    if let Some(v) = memo.borrow().get(&n) {
        return *v;
    }

    let res = dynamic_fibonacci(n - 1, memo.clone()) + dynamic_fibonacci(n - 2, memo.clone());
    memo.borrow_mut().insert(n, res);
    return res;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn dynamic_fibonacci_works() {
        let memo = Rc::new(RefCell::new(HashMap::new()));
        assert_eq!(1, dynamic_fibonacci(1, memo.clone()));
        assert_eq!(21, dynamic_fibonacci(8, memo.clone()));
        assert_eq!(34, dynamic_fibonacci(9, memo.clone()));
        assert_eq!(89, dynamic_fibonacci(11, memo.clone()));
        assert_eq!(144, dynamic_fibonacci(12, memo.clone()));
        assert_eq!(12586269025, dynamic_fibonacci(50, memo.clone()),);
    }
}

// fn dynamic_fibonacci(n: u128) -> u128 {
//     if n <= 2 {
//         return 1;
//     } else {
//         return dynamic_fibonacci(n - 1) + dynamic_fibonacci(n - 2);
//     }
// }
