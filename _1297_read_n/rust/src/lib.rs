use std::str;

struct File<'a> {
    cursor: usize,
    str:  &'a [u8],
}


impl<'a> File<'a> {
    fn new(str: &'a str) -> Self{
        Self {
            str: str.as_bytes(),
            cursor: 0,
        }
    }
    pub fn read_n(&mut self, n: usize) -> &'a str {
        // TODO: might see about either handling the error case, or using the ? operator.
        if self.cursor > self.str.len() {
            ""
        } else if self.cursor + n > self.str.len() {
            let slice = &self.str[self.cursor..];
            self.cursor += n;
            str::from_utf8(slice).unwrap()
        } else  {
            let slice = &self.str[self.cursor..n];
            self.cursor += n;
            str::from_utf8(slice).unwrap()
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    // Good morning! Here's your coding interview problem for today.
    //
    // This problem was asked by Microsoft.
    //
    // Using a read7() method that returns 7 characters from a file, implement readN(n) which reads n characters.
    //
    // For example, given a file with the content “Hello world”, three read7() returns “Hello w”, “orld” and then “”.
    #[test]
    fn test_7() {
        let expected_vec = vec!["Hello w", "orld", ""];
        let input = "Hello world";
        let mut file = File::new(input);
        let steps = 7;
        for (iteration, &expected) in expected_vec.iter().enumerate() {
            let actual = file.read_n(steps);
            assert_eq!(actual, expected, "ACTUAL {} DOES NOT EQUAL EXPECTED {} FOR ITERATION {}", actual, expected, iteration)
        }
    }
}
