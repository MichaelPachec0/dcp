use std::cmp::max;

fn simple_steps(steps: usize) -> usize {
    // This is basically fibonacci
    if (0..=2).contains(&steps) {
        return steps;
    }
    let len = steps + 1;
    let mut vec = Vec::<usize>::with_capacity(len);
    vec.push(1);
    vec.push(1);
    vec.push(2);
    for iteration  in 3..len {
        vec.push(vec[iteration-1] + vec[iteration-2]);
    }
    vec[steps]
}

#[cfg(test)]
mod tests {
    use super::*;
    //Good morning! Here's your coding interview problem for today.
    //
    // This problem was asked by Amazon.
    //
    // There exists a staircase with N steps, and you can climb up either 1 or 2 steps at a time.
    // Given N, write a function that returns the number of unique ways you can climb the staircase.
    // The order of the steps matters.
    //
    // For example, if N is 4, then there are 5 unique ways:
    //
    //      1, 1, 1, 1
    //      2, 1, 1
    //      1, 2, 1
    //      1, 1, 2
    //      2, 2
    // What if, instead of being able to climb 1 or 2 steps at a time,
    // you could climb any number from a set of positive integers X?
    // For example, if X = {1, 3, 5}, you could climb 1, 3, or 5 steps at a time.
    #[test]
    fn static_simple_climb() {
        let input = 4;
        let max_steps = 1;
        let expected = 5;
        let result = simple_steps(input);
        assert_eq!(result, expected, "RESULT {result} DOES NOT EQUAL EXPECTED {expected}");
    }
}
