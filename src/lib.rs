pub mod binary_search {
    use std::cmp::Ordering;

    pub fn binary_search<T>(slice: &[T], value: T) -> Option<T> where T: Ord + Copy {
        if slice.len() == 1 {
            return return_if_match(slice[0], value);
        }

        let mid = slice.len() / 2;
        let mid_val = slice[mid];

        return match value.cmp(&mid_val) {
            Ordering::Less => binary_search(&slice[..mid], value),
            Ordering::Greater => binary_search(&slice[mid..], value),
            Ordering::Equal => Some(mid_val)
        }
    }

    fn return_if_match<T>(actual: T, expected: T) -> Option<T> where T: PartialEq{
        if actual == expected {
            return Some(actual);
        }
        return None;
    }
}