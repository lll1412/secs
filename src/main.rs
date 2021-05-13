#![warn(clippy::all)]

use secs::World;

fn main() {
    let mut world = World::new();
    let entity = world.new_entity();
    world.add_component_to_entity(entity, Health(-10));
    world.add_component_to_entity(entity, Name("Tom"));

    let mut healths = world.borrow_component_vec_mut::<Health>().unwrap();
    let mut names = world.borrow_component_vec_mut::<Name>().unwrap();
    let zip = healths.iter_mut().zip(names.iter_mut());
    let iter = zip.filter_map(|(health, name)| Some((health.as_mut()?, name.as_mut()?)));

    iter.for_each(|(health, name)| {
        if health.0 < 0 {
            println!("{} has perished", name.0);
        };
    });
}

#[derive(Debug)]
struct Health(i32);

#[derive(Debug)]
struct Name(&'static str);
