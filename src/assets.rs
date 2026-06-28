use macroquad::prelude::*;

// Embeds atlas.png in the binary
const ATLAS_BYTES: &[u8] = include_bytes!("../assets/atlas.png");

pub struct Assets {
    pub atlas: Texture2D,
}

impl Assets {
    pub fn new() -> Assets {
        let atlas = Texture2D::from_file_with_format(ATLAS_BYTES, Some(ImageFormat::Png));
        atlas.set_filter(FilterMode::Nearest);

        Assets { atlas }
    }
}
