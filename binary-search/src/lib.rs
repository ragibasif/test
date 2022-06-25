pub fn binary_search(vec: Vec<i32>, target: i32) -> bool {
    let mut low = 0;
    let mut high = vec.len() - 1;
    let mut mid;
    while low <= high {
        mid = (low + high) / 2;
        if vec[mid] == target {
            return true;
        } else if vec[mid] > target {
            high = mid - 1;
        } else if vec[mid] < target {
            low = mid + 1;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_num() {
        let vec = vec![1, 2, 3, 4];
        assert_eq!(binary_search(vec, 1), true);
    }
    #[test]
    fn last_num() {
        let vec = vec![1, 2, 3, 4];
        assert_eq!(binary_search(vec, 4), true);
    }
    #[test]
    fn all_zeroes() {
        let vec = vec![0];
        assert_eq!(binary_search(vec, 0), true);
    }
    #[test]
    fn not_in() {
        let vec = vec![13, 515, 666, 67, 2];
        assert_eq!(binary_search(vec, 20), false);
    }
}
