use legion::*;
// use legion::world::SubWorld;
use sdl2::render::*;
use sdl2::pixels::Color;

#[system]
// #[read_component(Player)]
pub fn render(
    #[resource] canvas: &mut WindowCanvas,
    // world: &SubWorld,
) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    canvas.present();
}