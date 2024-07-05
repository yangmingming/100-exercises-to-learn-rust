// TODO: Given a vector of integers, leak its heap allocation.
//  Then split the resulting static slice into two halves and
//  sum each half in a separate thread.
//  Hint: check out `Vec::leak`.

use std::thread;

// leak 方法会将 Vec 的所有权泄露到堆上，使其生命周期变成 'static，从而允许在并发环境中安全地访问。
pub fn sum(v: Vec<i32>) -> i32 {
    let v = v.leak();
    let mid = v.len() / 2;
    let (left, right) = v.split_at(mid);
    let t1 = thread::spawn(move || left.iter().sum::<i32>());
    let t2 = thread::spawn(move || right.iter().sum::<i32>());

    t1.join().unwrap() + t2.join().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(sum(vec![]), 0);
    }

    #[test]
    fn one() {
        assert_eq!(sum(vec![1]), 1);
    }

    #[test]
    fn five() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn nine() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 45);
    }

    #[test]
    fn ten() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 55);
    }
}
