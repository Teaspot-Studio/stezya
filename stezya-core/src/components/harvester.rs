use glam::Vec2;
use legion::*;
use std::f32::consts::PI;

use super::pos::*;
use super::rot::*;
use super::size::*;
use super::vel::*;

/// Harvester component that has selected bool.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Harvester(pub bool);

/// Player size in pixels
pub const HARVESTER_SIZE: (f32,f32) = (20.0,20.0);

/// Player is circle for collision detection.
pub const HARVESTER_COLLIDE_RADIUS: f32 = 15.0;

/// Rotation speed in radians per second
pub const HARVESTER_ROTATE_SPEED: f32 = PI;

/// Possible inputs from player
#[derive(PartialEq, Eq, Hash)]
pub enum HarvesterInput {
    Move
   ,Stop
}

/// Allocate components for the player
pub fn create_harvester(resources: &mut Resources, w: &mut World) -> Entity {
    let ws: WorldSize = *resources.get::<WorldSize>().unwrap();
    w.push((
        Harvester(false),
        Pos(Vec2::new(
            ws.0 as f32 * 0.5,
            ws.1 as f32 * 0.5,
        )),
        Rot(0.0),
        Vel(Vec2::new(0.0,0.0)),
    ))
}
