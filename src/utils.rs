/*
MIT License

Copyright (c) 2024 Anthony Rubick

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

use anyhow::{anyhow, bail, Result};

/// Read a bit vector from stdin
///
/// The input is expected to be a string of 0s and 1s
///
/// Side effects:
/// - Reads from stdin
///
/// # Errors
/// - if there is an error reading from stdin
/// - if the input contains a character other than 0 or 1
pub fn read_bit_vec_from_stdin() -> Result<Vec<u8>> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    bit_vec_from_string(&input)
}

/// Convert a string of 0s and 1s to a bit vector of 1s and 0s (u8s)
///
/// # Errors
/// - if the input contains a character other than 0 or 1
#[allow(clippy::cast_possible_truncation)]
pub fn bit_vec_from_string(s: &str) -> Result<Vec<u8>> {
    s.trim()
        .chars()
        .map(|c| match c {
            '0' | '1' => Ok(c
                .to_digit(10)
                .ok_or_else(|| anyhow!("Failed to convert char to digit"))?
                as u8),
            _ => bail!("Invalid character in input, expected 0 or 1"),
        })
        .collect()
}

/// Convert a slice of bits to a 32-bit integer
/// The bits are assumed to be in big-endian order, i.e. the first bit is the most significant
/// bit and the last bit is the least significant bit
///
/// the output will be in little-endian order, i.e. the first bit is the least significant bit
///
/// in general, it'll output will be of reverse endian-ness to the input
///
/// if the input is less than 32 bits, the output will be zero extended
/// if the input is more than 32 bits, the output will be truncated to the least significant 32 bits
#[must_use]
pub fn bit_vec_to_int(bits: &[u8]) -> u32 {
    bits.iter()
        .rev()
        .enumerate()
        .take(32)
        .map(|(place, bit)| u32::from(*bit) << place)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bit_vec_to_int() {
        // test 32 bits
        assert_eq!(
            bit_vec_to_int(&[
                1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0,
                1, 0, 1, 0,
            ]),
            0xAAAA_AAAA,
            "32-bit binary number"
        );
        // test less than 32 bits
        assert_eq!(bit_vec_to_int(&[1]), 1, "1-bit binary number");
        assert_eq!(bit_vec_to_int(&[0]), 0, "1-bit binary number");
        // test more than 32 bits
        assert_eq!(
            bit_vec_to_int(&[
                1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0,
                1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0,
                1, 0, 1, 0, 1, 0, 1, 0,
            ]),
            0xAAAA_AAAA,
            "more than 32-bits"
        );
    }
}
