
///
///
/// # Arguments
///
/// * `steps`: The number of stairs.
///
/// returns: usize
///
/// # Examples
///
/// ```
///
/// ```
fn simple_steps(steps: usize) -> usize {
    // This is basically fibonacci
    if (0..=2).contains(&steps) {
        return steps;
    }
    let len = steps + 1;
    let mut result = 0;
    let mut vec = Vec::<usize>::with_capacity(2);
    vec.push(1);
    vec.push(1);
    for _ in 2..len {
        result = vec.iter().fold(0, |acc, &i| acc + i);
        vec[0] = vec[1];
        vec[1] = result;
    }
    result
}


///
///
/// # Arguments
///
/// * `steps`: The number of stairs
/// * `step_by_iter`: The number of stairs we can climb at a time.
///
/// returns: usize
///
/// # Examples
///
/// ```
///
/// ```
fn complex_steps(steps: usize, step_by_iter: Vec<usize>) -> usize {
    // Theory: we can reduce the length to len of steps_by_iter and use len^2 as the size of the vec.
    // as far I can see the code will chunk itself eventually into groups.
    // (when steps is sufficiently long enough)
    let len = steps + 1;
    let mut vec = Vec::<usize>::with_capacity(len);
    vec.push(1);
    for i in 1..len {
        let mut intermediary = vec![];
        for &step_by in &step_by_iter {
            if i.checked_sub(step_by).is_none() {
                continue
            }
            let index = i - step_by;
            let x  = vec[index];
            intermediary.push(x)
        }
        let number = intermediary.iter().fold(0, |acc, i| acc + i);
        vec.push(number);
    }
    vec[steps]
}

#[cfg(test)]
mod tests {
    use super::*;
    // Good morning! Here's your coding interview problem for today.
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
        let expected = 5;
        let result = simple_steps(input);
        assert_eq!(
            result, expected,
            "RESULT {result} DOES NOT EQUAL EXPECTED {expected}"
        );
    }
    #[test]
    fn static_complex_climb() {
        let input = 22;
        let step_by = vec![1,3,4,7];
        let result = complex_steps(input, step_by);
        println!("{result}");
    }
}
