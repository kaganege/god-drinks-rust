//! The program god-drinks-rust implements an application that creates an empty
//! simulated world with no meaning or purpose.

use world::animals::TabbyCat;
use world::geometry::{Circle, PointSet, Sequence, SineWave};
use world::lovable::{Lovable, OpinionError::*};
use world::vegetables::{Eggplant, Tomato};
use world::{Thing, ToAttribute, ToExecution, World};

mod world;

fn main() {
  let mut me = Lovable::new("Me", 0, true, -1, false);
  let mut you = Lovable::new("You", 0, false, -1, false);

  let mut world = World::new(5);
  world.add_thing(Box::new(me.clone()));
  world.add_thing(Box::new(you.clone()));
  world.start_simulation();

  if let Some(me) = <Lovable as Thing>::as_mut_any(&mut me).downcast_mut::<PointSet>() {
    you.add_attribute(me.dimensions().to_attribute());
    me.reset_dimensions();
  }

  if let Some(me) = <Lovable as Thing>::as_mut_any(&mut me).downcast_mut::<Circle>() {
    you.add_attribute(me.circumference().to_attribute());
    me.reset_circumference();
  }

  if let Some(me) = <Lovable as Thing>::as_mut_any(&mut me).downcast_mut::<SineWave>() {
    you.add_action("sit", &me.tangent(you.x_position()));
  }

  if let Some(me) = <Lovable as Thing>::as_mut_any(&mut me).downcast_mut::<Sequence>() {
    me.set_limit(you.to_limit());
  }

  me.toggle_current();

  me.set_can_see(false);
  me.add_feeling("dizzy");

  world.time_travel_for_two("AD", 617, &me, &you);
  world.time_travel_for_two("BC", 3691, &me, &you);

  world.unite(&me, &you);

  if me.get_num_stimulations_available() >= you.get_num_stimulations_needed() {
    you.set_satisfaction(me.to_satisfaction());
  }

  if me.feeling_index("happy").is_some() {
    me.request_execution(&world);
  }

  world.lock_thing(&me);
  world.lock_thing(&you);

  if let Some(me) = <Lovable as Thing>::as_mut_any(&mut me).downcast_mut::<Eggplant>() {
    you.add_attribute(me.nutrients().to_attribute());
    me.reset_nutrients();
  }

  if let Some(me) = <Lovable as Thing>::as_mut_any(&mut me).downcast_mut::<Tomato>() {
    you.add_attribute(me.antioxidants().to_attribute());
    me.reset_antioxidants();
  }

  if let Some(me) = <Lovable as Thing>::as_mut_any(&mut me).downcast_mut::<TabbyCat>() {
    me.purr();
  }

  if world.god().eq(&me) {
    you.set_proof(me.to_proof());
  }

  me.toggle_gender();

  world.procreate(&me, &you);

  me.toggle_role_bdsm();

  world.make_high(&me);
  world.make_high(&you);

  if me.sense_index("vibration").is_some() {
    me.add_feeling("complete");
  }

  world.unlock(&you);
  world.remove_thing(&you);

  me.look_for(&you, &world);
  me.look_for(&you, &world);
  me.look_for(&you, &world);
  me.look_for(&you, &world);
  me.look_for(&you, &world);

  if me.memory().is_erasable() {
    me.remove_feeling("disheartened");
  }

  if let Err(e) = me.set_opinion(me.opinion_index("you are here").unwrap(), false) {
    match e {
      IllegalArgument => world.announce("God is always true"),
      e => panic!("{e:?}"),
    }
  }

  world.run_execution();
  world.run_execution();
  world.run_execution();
  world.run_execution();
  world.run_execution();
  world.run_execution();
  world.run_execution();
  world.run_execution();
  world.run_execution();
  world.run_execution();
  world.run_execution();
  world.run_execution();

  world.announce(("1", "de"));
  world.announce(("2", "es"));
  world.announce(("3", "fr"));
  world.announce(("4", "kr"));
  world.announce(("5", "se"));
  world.announce(("6", "cn"));

  world.run_execution();

  if world.is_executable_by(&me) {
    you.set_execution(me.to_execution());
  }

  if world.thing_index(&me).is_some() {
    world.run_execution();
  }

  me.escape(&world);

  me.learn_topic("love");
  me.take_exam_topic("love");
  me.get_algebraic_expression("love");
  me.escape("love");

  world.execute(me);
}
