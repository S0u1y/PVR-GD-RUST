extern crate public;
mod entity;
mod player;
mod stats;
mod world;
mod inventory_slot;
mod mob;
mod base_container;
mod context_options;

use std::ops::{Deref, DerefMut};
use godot::classes::{ISprite2D, Sprite2D};
use godot::prelude::*;
use crate::entity::{IEntity, TargetAction};
use crate::stats::Stats;
use crate::world::World;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
