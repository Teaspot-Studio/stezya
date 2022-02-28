use legion::*;

fn init_resources() -> Resources {
    let mut resources = Resources::default();

    resources
}

pub fn init_world() -> (World, Resources) {
    let mut world = World::default();
    let mut resources = init_resources();

    (world, resources)
}

pub fn init_systems() -> Result<legion::systems::Builder, String> {
    let schedule = Schedule::builder();
        // .add_system(player_system())
        // .add_system(bullet_system())
        // .add_system(asteroid_system())
        // .add_system(physics_system())
        // .flush()
        // .add_thread_local(audio_system(AudioReses::init()?))
        // .add_thread_local(render_system())
        // .build();
    Ok(schedule)
}