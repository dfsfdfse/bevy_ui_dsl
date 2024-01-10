//! This crate simplifies the process of creating widgets in bevy using a simple extensible DSL.

#[cfg(feature = "class_helpers")]
pub mod class_helpers;
mod widgets;

use bevy_ecs::bundle::Bundle;
use bevy_ecs::system::{EntityCommands, Resource};
use bevy_ecs::{all_tuples, entity::Entity, world::World};
use bevy_hierarchy::{BuildChildren, ChildBuilder};
pub use widgets::*;

/// Wrapper for [`ChildBuilder`] that also propogates an [`AssetServer`] for the children that need it.
// It has enough ' for a lifetime ;)
pub struct UiChildBuilder<'a, 'b, 'c, 'd> {
    builder: &'a mut ChildBuilder<'b, 'c, 'd>,
    world: &'a World,
}

impl<'a, 'b, 'c, 'd> UiChildBuilder<'a, 'b, 'c, 'd> {
    pub fn spawn(&mut self, bundle: impl Bundle) -> UiEntityCommands<'a, 'b, 'c, '_> {
        let commands: EntityCommands<'b, 'c, '_> = self.builder.spawn(bundle);
        UiEntityCommands {
            world: self.world,
            commands,
        }
    }
}

/// Wrapper for [`EntityCommands`] that also propagates an [`AssetServer`] for the children that need it.
pub struct UiEntityCommands<'a, 'b, 'c, 'd> {
    commands: EntityCommands<'b, 'c, 'd>,
    world: &'a World,
}

impl<'a, 'b, 'c, 'd> UiEntityCommands<'a, 'b, 'c, 'd> {
    pub fn id(&self) -> Entity {
        self.commands.id()
    }
    pub fn insert(&mut self, bundle: impl Bundle) -> &mut Self {
        self.commands.insert(bundle);
        self
    }
    pub fn with_children(mut self, spawn_children: impl FnOnce(&mut UiChildBuilder)) -> Self {
        self.commands.with_children(|builder| {
            let mut ui_builder = UiChildBuilder {
                world: self.world,
                builder,
            };
            spawn_children(&mut ui_builder);
        });
        self
    }
}

/// Something that can overwrite a value, typically a node bundle.

pub trait Class<P> {
    type In;
    fn apply(self, b: &mut Self::In, world: &World);
}

macro_rules! impl_class_tuple {
    ($($P: ident),*) => {
        impl<B, F, $($P),*> Class<(B, $($P,)*)> for F
        where
            F: FnOnce(&mut B, $(& $P), *),
            $($P: Resource,)*
        {
            type In = B;
            fn apply(self, b: &mut B, world: &World) {
                self(b, $(world.resource::<$P>(),)*);
            }
        }
    }
}

all_tuples!(impl_class_tuple, 0, 5, P);

macro_rules! impl_class_more_tuple {
    ($(($P: ident, $p: ident)),*) => {
        #[allow(non_snake_case)]
        impl<B, $($P, $p),*> Class<(B, $($P,)*)> for ($($p,)*)
        where
            $($p: Class<$P, In = B>,)*
        {
            type In = B;
            fn apply(self, b: &mut Self::In, world: &World) {
                let ($($p,)*) = self;
                $($p.apply(b, world);)*
            }
        }
    };
}

all_tuples!(impl_class_more_tuple, 0, 5, P, S);

/// Adds a helper method to [`Entity`] that allows it to be sent to an [`Option`][`Entity`]
/// ergonomically.
pub trait EntityWriter {
    fn set(self, entity: &mut Option<Entity>);
    fn push(self, destination: &mut Vec<Entity>);
}

impl EntityWriter for Entity {
    /// Copies this entity into an Option.
    fn set(self, entity: &mut Option<Entity>) {
        *entity = Some(self);
    }
    /// Pushes a copy of this Entity into a Vec.
    fn push(self, entities: &mut Vec<Entity>) {
        entities.push(self);
    }
}
