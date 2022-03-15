use glam::Vec2;
use legion::*;
use legion::world::SubWorld;
use std::collections::HashSet;

use super::super::components::harvester::*;
use super::super::components::pos::*;
use super::super::components::rot::*;
use super::super::components::size::*;
use super::super::components::time::*;
use super::super::components::vel::*;

/// System that updates position, calcs space warping and collisions.
#[system]
#[write_component(Harvester)]
#[write_component(Pos)]
#[write_component(Vel)]
#[write_component(Rot)]
pub fn harvester(
    #[resource] hinput: &HashSet<HarvesterInput>,
    #[resource] delta: &DeltaTime,
    #[resource] ws: &WorldSize,
    mut world: &mut SubWorld,
) {
    let dt = delta.0.as_secs_f32();
    for (harvester, pos, vel, rot) in <(&mut Harvester, &Pos, &mut Vel, &mut Rot)>::query().iter_mut(world)
    {
        // Drop thrusting flag. That disables rendering of engine when there is not pressed up key by player.
        harvester.0 = false;

        // Process inputs
        
        for ie in hinput.iter() {
            match ie {
                HarvesterInput::Move => {
                    add_vel_forward(vel, rot, 10.0)
                }
                HarvesterInput::Stop => {
                    stop_vel(vel, rot)
                }

            }
        }
    }

}
