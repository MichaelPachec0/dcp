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

}
