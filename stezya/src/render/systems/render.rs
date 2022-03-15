use legion::*;
use legion::world::SubWorld;
use sdl2::rect::Rect;
use sdl2::render::*;
use sdl2::pixels::Color;

use stezya_core::components::harvester::Harvester;
use stezya_core::components::pos::Pos;

use crate::render::components::spriteview::SpriteView;





#[system]
#[read_component(Harvester)]
#[read_component(SpriteView)]
#[read_component(Pos)]
pub fn render(
    #[resource] canvas: &mut WindowCanvas,
    world: &SubWorld,
) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    let mut harvester_query = <(&Harvester, &Pos)>::query();
      for (harvester, pos) in harvester_query.iter(world) {
          render_harvester(canvas, harvester, pos);
      }

    canvas.present();
}

fn render_harvester(canvas: &mut WindowCanvas, harvester: &Harvester, pos: &Pos) {
    let mut harvreqt = Rect::new(pos.0.x as i32, pos.0.y as i32,20,20);
    canvas.set_draw_color(Color::RGB(255,0,0));
    canvas.fill_rect(harvreqt);
}


fn render_harvester_2(canvas: &mut WindowCanvas, harvester: &Harvester, pos: &Pos, sv: &SpriteView) {
    let mut harvreqt = Rect::new(pos.0.x as i32, pos.0.y as i32,sv.0,sv.1);
    canvas.set_draw_color(Color::RGB(255,0,0));
    canvas.fill_rect(harvreqt);
}
