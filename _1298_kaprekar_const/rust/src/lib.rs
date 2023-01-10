use std::num::ParseIntError;
use itertools::{Itertools};

const KAPREKAR: i32 = 6174;

pub fn kaprekar_constant(number: i32) -> Result<i32, ParseIntError> {
    let mut iterations = 0;

    let mut number = number;
    loop {
        number = calculate(number)?;
        if number == KAPREKAR {
            return Ok(iterations+1);
        } else if iterations == 100 {
            return Ok(-1);
        }
        iterations += 1;
    }
}

fn calculate(number: i32) -> Result<i32, ParseIntError> {
    let num_str = number.to_string();
    let iter = num_str.chars().sorted();
    let desc_str = iter.clone().rev().collect::<String>();
    let asc_str = iter.collect::<String>();
    Ok(desc_str.parse::<i32>()? - asc_str.parse::<i32>()?)
}

#[cfg(test)]
mod tests {
    use rand::Rng;
    use super::*;
    use rand::rngs::ThreadRng;

    // Good morning! Here's your coding interview problem for today.
    //
    // This problem was asked by Salesforce.
    //
    // The number 6174 is known as Kaprekar's contant, after the mathematician who discovered an
    // associated property: for all four-digit numbers with at least two distinct digits, repeatedly
    // applying a simple procedure eventually results in this value. The procedure is as follows:
    //
    //     For a given input x, create two new numbers that consist of the digits in x in ascending
    //     and descending order.
    //
    //     Subtract the smaller number from the larger number.
    //
    // For example, this algorithm terminates in three steps when starting from 1234:
    //
    //     4321 - 1234 = 3087
    //     8730 - 0378 = 8352
    //     8532 - 2358 = 6174
    //
    // Write a function that returns how many steps this will take for a given input N.
    #[test]
    fn static_number() -> Result<(), Box<dyn std::error::Error>> {
        let input = 1234;
        let expected = 3;
        let initial = input;
        let result = kaprekar_constant(input)?;
        assert_eq!(expected, result, "EXPECTED ITERATIONS {expected} DOES NOT EQUAL RESULT {result}");
        Ok(())
    }


    #[test]
    fn random_numbers() -> Result<(), Box<dyn std::error::Error>> {
        let iterations = 32;
        let gen = RandGen {
            start: 1000,
            stop: 9999,
            iterations,
            rand_gen: rand::thread_rng(),
        };
        for (i, number) in gen.into_iter().enumerate() {
            println!("ITERATION {} of {iterations} NUM {number}", i+1);
            let steps = kaprekar_constant(number)?;
            assert_ne!(steps, -1, "FAILED TO CALCULATE THE NUMBER OF STEPS FOR NUMBER {number}");
            println!("AWNSER {steps}");
        }
        Ok(())
    }

    struct RandGen {
        start: i32,
        stop: i32,
        iterations: i32,
        rand_gen: ThreadRng,
    }

    impl Iterator for RandGen {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.iterations == 0 {
                None
            } else {
                self.iterations -= 1;
                Some(self.rand_gen.gen_range(self.start..self.stop))
            }
        }
    }
}
