#[macro_use]
extern crate public;
mod entity;
mod player;
mod stats;
mod world;
mod inventory_slot;
mod mob;
mod ContextOption;

use std::ops::{Deref, DerefMut};
use godot::classes::{ISprite2D};
use godot::prelude::*;
use crate::entity::{IEntity};

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}


