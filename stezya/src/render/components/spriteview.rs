
#[derive(Copy, Clone, Debug)]
pub struct SpriteView(pub u32, pub u32);

impl Default for SpriteView {
    fn default() -> Self {
        SpriteView(20, 20)
    }
}
