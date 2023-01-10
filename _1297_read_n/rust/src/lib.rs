//! Testing for random strings
use std::str;

struct File<'a> {
    cursor: usize,
    str: &'a [u8],
}

impl<'a> File<'a> {
    fn new(str: &'a str) -> Self {
        Self {
            str: str.as_bytes(),
            cursor: 0,
        }
    }
    pub fn read_n(&mut self, n: usize) -> Result<&'a str, str::Utf8Error> {
        if self.cursor > self.str.len() {
            Ok("")
        } else if self.cursor + n > self.str.len() {
            let slice = &self.str[self.cursor..];
            self.cursor += n;
            str::from_utf8(slice)
        } else {
            let slice = &self.str[self.cursor..n + self.cursor];
            self.cursor += n;
            str::from_utf8(slice)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::distributions::{Alphanumeric, DistString};
    use rand::rngs::ThreadRng;
    use rand::Rng;
    use unicode_segmentation::UnicodeSegmentation;
    // Good morning! Here's your coding interview problem for today.
    //
    // This problem was asked by Microsoft.
    //
    // Using a read7() method that returns 7 characters from a file, implement readN(n) which reads n characters.
    //
    // For example, given a file with the content “Hello world”, three read7() returns “Hello w”, “orld” and then “”.
    #[test]
    fn static_string() -> Result<(), Box<dyn std::error::Error>> {
        let expected_vec = vec!["Hello w", "orld", ""];
        let input = "Hello world";
        let mut file = File::new(input);
        let steps = 7;
        for (iteration, &expected) in expected_vec.iter().enumerate() {
            let actual = file.read_n(steps)?;
            assert_eq!(
                actual, expected,
                "ACTUAL {actual} DOES NOT EQUAL EXPECTED {expected} FOR ITERATION {iteration}"
            );
        }
        Ok(())
    }
    #[test]
    fn static_len_rand_str() -> Result<(), Box<dyn std::error::Error>> {
        let mut rng_thrd = rand::thread_rng();
        let steps = 7;
        let (strings, expected_strings) = gen_input(&mut rng_thrd, steps, None);
        check(&strings, &expected_strings, steps)
    }

    #[test]
    /// Testing for random strings with random lengths.
    fn rand_len_rand_str() -> Result<(), Box<dyn std::error::Error>> {
        let mut rng_thrd = rand::thread_rng();
        let steps = rng_thrd.gen_range(5..20);
        let (strings, expected_strings) = gen_input(&mut rng_thrd, steps, Some((0, 50)));
        check(&strings, &expected_strings, steps)
    }

    fn gen_input(
        rng: &mut ThreadRng,
        steps: usize,
        option_range: Option<(usize, usize)>,
    ) -> (Vec<String>, Vec<Vec<String>>) {
        let strings = (0..12)
            .into_iter()
            .map(|_| {
                let len = match option_range {
                    Some((start, stop)) => rng.gen_range(start..stop),
                    _ => steps * 3,
                };
                Alphanumeric.sample_string(rng, len)
            })
            .collect::<Vec<String>>();
        let expected_strings = strings
            .iter()
            .map(|str| chunk_helper(str, steps))
            .collect::<Vec<Vec<String>>>();
        (strings, expected_strings)
    }

    fn check(strings: &[String], expected_strings: &[Vec<String>], steps: usize) -> Result<(), Box<dyn std::error::Error>> {
        for (str_iteration, string) in strings.iter().enumerate() {
            println!(
                "Iteration {} of {}, string: {}",
                str_iteration + 1,
                strings.len(),
                string
            );
            let mut file_struct = File::new(string.as_str());
            for (iteration, expected) in expected_strings[str_iteration].iter().enumerate() {
                let actual = file_struct.read_n(steps)?;
                assert_eq!(
                    actual, expected,
                    "ACTUAL {actual} DOES NOT EQUAL EXPECTED {expected} FOR ITERATION {iteration}"
                );
            }
        }
        Ok(())
    }

    fn chunk_helper(str: &String, size: usize) -> Vec<String> {
        let tmp_vec = str.graphemes(true).collect::<Vec<&str>>();
        let mut str_vec = tmp_vec
            .chunks(size)
            .map(|vec| vec.join(""))
            .collect::<Vec<String>>();
        str_vec.push(String::new());
        str_vec
    }
}
