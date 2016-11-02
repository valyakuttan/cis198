// Problem 1

/// Computes the sum of all elements in the input i32 slice named `slice`
pub fn sum(slice: &[i32]) -> i32 {
    let mut sum = 0;

    for i in slice {
        sum += *i;
    }

    sum
}

/// Deduplicates items in the input vector `vs`. Produces a vector containing
/// the first instance of each distinct element of `vs`, preserving the
/// original order.
pub fn dedup(vs: &[i32]) -> Vec<i32> {
    let mut v = Vec::new();

    for e in vs {
        if !v.contains(e) {
            v.push(*e)
        }
    }

    v
}

/// Filters a vector `vs` using a predicate `pred` (a function from `i32` to
/// `bool`). Returns a new vector containing only elements that satisfy `pred`.
pub fn filter(vs: &[i32], pred: &Fn(i32) -> bool) -> Vec<i32> {
    let mut v = Vec::new();

    for e in vs {
        if pred(*e) {
            v.push(*e)
        }
    }

    v
}
