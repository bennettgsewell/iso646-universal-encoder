/// Maps a Unicode character to its ISO-646 equivalent.
///
/// # Fields
/// * `char` - Unicode
/// * `u8` - ISO-646
#[derive(Clone, Copy)]
pub struct Map(char, u8);

pub enum EncodingVariant {
    US,
    /// Canada
    CA,
}

const US: [Map; 0] = [];
const CA: [Map; 1] = [Map('x', 5u8)];

pub fn get_code_map(variant: EncodingVariant) -> &'static [Map] {
    match variant {
        EncodingVariant::US => &US, 
        EncodingVariant::CA => &CA,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn my_test() {
    }
}