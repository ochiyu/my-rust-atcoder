pub fn bisect_right<T>(a:&Vec<T>, x:T)->usize where T:PartialOrd {
    let mut l = 0;
    let mut r = a.len();
    while l < r {
        let mid = (l + r) / 2;
        if x < a[mid] {
            r = mid;
        } else {
            l = mid + 1;
        }
    }
    return l;
}

pub fn bisect_left<T>(a:&Vec<T>, x:T)->usize where T:PartialOrd {
    let mut l = 0;
    let mut r = a.len();
    while l < r {
        let mid = (l + r) / 2;
        if x > a[mid] {
            l = mid + 1;
        } else {
            r = mid;
        }
    }
    return l;
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bisect_right_empty() {
        let a = vec![];
        assert_eq!(bisect_right(&a, 0), 0);
        assert_eq!(bisect_right(&a, 1), 0);
        assert_eq!(bisect_right(&a, 2), 0);
        assert_eq!(bisect_right(&a, 3), 0);
        assert_eq!(bisect_right(&a, 4), 0);
        assert_eq!(bisect_right(&a, 5), 0);
        assert_eq!(bisect_right(&a, 6), 0);
    }

    #[test]
    fn test_bisect_right_sequence() {
        let a = vec![1, 2, 3, 4, 5];
        assert_eq!(bisect_right(&a, 0), 0);
        assert_eq!(bisect_right(&a, 1), 1);
        assert_eq!(bisect_right(&a, 2), 2);
        assert_eq!(bisect_right(&a, 3), 3);
        assert_eq!(bisect_right(&a, 4), 4);
        assert_eq!(bisect_right(&a, 5), 5);
        assert_eq!(bisect_right(&a, 6), 5);
    }

    #[test]
    fn test_bisect_right_duplication() {
        let a = vec![1, 1, 2, 2, 3];
        assert_eq!(bisect_right(&a, 0), 0);
        assert_eq!(bisect_right(&a, 1), 2);
        assert_eq!(bisect_right(&a, 2), 4);
        assert_eq!(bisect_right(&a, 3), 5);
        assert_eq!(bisect_right(&a, 4), 5);
        assert_eq!(bisect_right(&a, 5), 5);
        assert_eq!(bisect_right(&a, 6), 5);
    }

    #[test]
    fn test_bisect_left_empty() {
        let a = vec![];
        assert_eq!(bisect_left(&a, 0), 0);
        assert_eq!(bisect_left(&a, 1), 0);
        assert_eq!(bisect_left(&a, 2), 0);
        assert_eq!(bisect_left(&a, 3), 0);
        assert_eq!(bisect_left(&a, 4), 0);
        assert_eq!(bisect_left(&a, 5), 0);
        assert_eq!(bisect_left(&a, 6), 0);
    }

    #[test]
    fn test_bisect_left_sequence() {
        let a = vec![1, 2, 3, 4, 5];
        assert_eq!(bisect_left(&a, 0), 0);
        assert_eq!(bisect_left(&a, 1), 0);
        assert_eq!(bisect_left(&a, 2), 1);
        assert_eq!(bisect_left(&a, 3), 2);
        assert_eq!(bisect_left(&a, 4), 3);
        assert_eq!(bisect_left(&a, 5), 4);
        assert_eq!(bisect_left(&a, 6), 5);
    }

    #[test]
    fn test_bisect_left_duplication() {
        let a = vec![1, 1, 2, 2, 3];
        assert_eq!(bisect_left(&a, 0), 0);
        assert_eq!(bisect_left(&a, 1), 0);
        assert_eq!(bisect_left(&a, 2), 2);
        assert_eq!(bisect_left(&a, 3), 4);
        assert_eq!(bisect_left(&a, 4), 5);
        assert_eq!(bisect_left(&a, 5), 5);
        assert_eq!(bisect_left(&a, 6), 5);
    }
}