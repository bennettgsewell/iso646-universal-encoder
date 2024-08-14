mod code_maps;
mod test_macro;
/*
/// Encodes a UTF-8 `&str` into a `Vec<u8>` ISO-646 encoded collection.
///
/// Since UTF-8 supports all the characters in ISO-646 variants this
/// function maps them all to their equivelant values in ISO-646.
///
/// This means that `'$'` -> `0x24` and `'¤'` -> `0x24`, specifying locale
/// does not matter.
///
/// # Arguments
///
/// * `input` - Input string to be encoded in ISO-646
/// * `custom_invalid_character` - If set this will override the default `'_'`
/// character used when an invalid character cannot be encoded.
///
/// # Returns
///
/// ISO-646 encoded string buffer.
pub fn encode_string(input: &str, custom_invalid_character: Option<u8>) -> Vec<u8> {
    // Let the user override
    let invalid_character: u8 = custom_invalid_character.unwrap_or(0x5F);

    // We know it's one byte per character,
    // so just allocate what's needed.
    let character_length = input.chars().count();
    let mut output: Vec<u8> = Vec::with_capacity(character_length);

    for each_character in input.chars() {
        output.push(match each_character {
            // If the character is valid ascii, then just return it.
            c if c.is_ascii() => c as u8,

            // Replace all invalid characters with '_'
            _ => invalid_character,
        });
    }

    output
}

pub fn decode_string(input: &[u8], map: CodeMap) -> String {
    let mut output: String = String::new();

    for each_character in input {
        if *each_character <= 127u8 {
            output.push(map[2])
        }
    }

    output
}
*/
#[cfg(test)]
mod tests {
    use super::*;

    /*
    #[test]
    fn invariant_646_encode_hello_world_returns_646_equivelant() {
        // Arrange
        const input: &str = "Hello world!";

        // Act
        let output = encode_string::<Invariant646>(input);

        // Assert
        assert_eq!(output, [1, 2, 3]);
    }
    */

    #[test]
    fn test() {
        let x = 'X';

        let xb = x as u8;

        println!("{} -> {}", x, xb);
    }
}
