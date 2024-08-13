/// Maps a Unicode character to its ISO-646 equivalent.
///
/// # Fields
/// * `char` - Unicode
/// * `u8` - ISO-646
struct Map(char, u8);

impl Map {
    /// `NUL` character
    pub fn default() -> Map {
        Map(0u8 as char, 0u8)
    }
}

impl Clone for Map {
    fn clone(&self) -> Self {
        Self(self.0.clone(), self.1.clone())
    }
}

impl Copy for Map {}

const CODE_MAP_LENGTH: usize = 127;

/// 0x00 through 0x7F characters in the ISO-646 standard
type CodeMap = [Map; CODE_MAP_LENGTH];

/// Returns a default map of Unicode characters to their associated ISO-646 bytes.
fn get_default() -> CodeMap {
    let mut code_map: CodeMap = [Map::default(); CODE_MAP_LENGTH];

    for i in 0..CODE_MAP_LENGTH {
        let byte = i as u8;

        code_map[i] = Map(byte as char, byte as u8);
    }

    code_map
}

