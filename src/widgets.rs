use super::{Class, UiChildBuilder};
use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::Bundle;
use bevy_ecs::system::Commands;
use bevy_ecs::world::World;
use bevy_hierarchy::BuildChildren;
use bevy_text::{TextSection, TextStyle};
use bevy_ui::node_bundles::{ButtonBundle, ImageBundle, NodeBundle, TextBundle};
use bevy_ui::{AlignItems, FlexWrap, JustifyContent, Style, Val};

/// Spawns a [`NodeBundle`] as the root with children.
pub fn root<P>(
    class: impl Class<P, In = NodeBundle>,
    world: &World,
    commands: &mut Commands,
    children: impl FnOnce(&mut UiChildBuilder),
) -> Entity {
    rooti(class, world, commands, (), children)
}

/// Spawns a [`NodeBundle`] as the root with children.
pub fn rooti<P>(
    class: impl Class<P, In = NodeBundle>,
    world: &World,
    commands: &mut Commands,
    extras: impl Bundle,
    children: impl FnOnce(&mut UiChildBuilder),
) -> Entity {
    let mut bundle = NodeBundle::default();
    class.apply(&mut bundle, world);
    commands
        .spawn((bundle, extras))
        .with_children(|builder| {
            let mut builder = UiChildBuilder { builder, world };
            children(&mut builder);
        })
        .id()
}

/// Spawns a clear [`NodeBundle`] that takes up the full space of its parent.
/// Often required for embedding other widgets after the initial widget is spawned.
pub fn blank<P>(
    parent: Entity,
    class: impl Class<P, In = NodeBundle>,
    world: &World,
    commands: &mut Commands,
    children: impl FnOnce(&mut UiChildBuilder),
) -> Entity {
    blanki(parent, class, world, commands, (), children)
}

/// Spawns a clear [`NodeBundle`] that takes up the full space of its parent.
/// Often required for embedding other widgets after the initial widget is spawned.
pub fn blanki<P>(
    parent: Entity,
    class: impl Class<P, In = NodeBundle>,
    world: &World,
    commands: &mut Commands,
    extras: impl Bundle,
    children: impl FnOnce(&mut UiChildBuilder),
) -> Entity {
    commands
        .entity(parent)
        .with_children(|builder| {
            let mut bundle = NodeBundle::default();
            class.apply(&mut bundle, world);
            let mut builder = UiChildBuilder { builder, world };
            builder.spawn((bundle, extras)).with_children(children);
        })
        .id()
}

/// Spawns a [`NodeBundle`] with children.
pub fn node<P>(
    class: impl Class<P, In = NodeBundle>,
    parent: &mut UiChildBuilder,
    children: impl FnOnce(&mut UiChildBuilder),
) -> Entity {
    nodei(class, (), parent, children)
}

/// Spawns a [`NodeBundle`] with children.
pub fn nodei<P>(
    class: impl Class<P, In = NodeBundle>,
    extras: impl Bundle,
    parent: &mut UiChildBuilder,
    children: impl FnOnce(&mut UiChildBuilder),
) -> Entity {
    let mut bundle = NodeBundle::default();
    class.apply(&mut bundle, parent.world);

    let mut commands = parent.spawn(bundle);
    commands.insert(extras);
    commands.with_children(children).id()
}

/// Spawns a [`TextBundle`].
pub fn text<P, P1>(
    text: impl Into<String>,
    class: impl Class<P, In = TextBundle>,
    text_class: impl Class<P1, In = TextStyle>,
    parent: &mut UiChildBuilder,
) -> Entity {
    texti(text, class, text_class, (), parent)
}

/// Spawns a [`TextBundle`].
pub fn texti<P, P1>(
    text: impl Into<String>,
    class: impl Class<P, In = TextBundle>,
    text_class: impl Class<P1, In = TextStyle>,
    extras: impl Bundle,
    parent: &mut UiChildBuilder,
) -> Entity {
    let mut bundle = TextBundle::default();
    class.apply(&mut bundle, parent.world);
    let sections = &mut bundle.text.sections;
    let mut style = TextStyle::default();
    text_class.apply(&mut style, parent.world);
    sections.push(TextSection {
        value: text.into(),
        style,
    });
    parent.spawn((bundle, extras)).id()
}

/// Spawns a [`ButtonBundle`] with children.
pub fn button<P>(
    class: impl Class<P, In = ButtonBundle>,
    parent: &mut UiChildBuilder,
    children: impl FnOnce(&mut UiChildBuilder),
) -> Entity {
    buttoni(class, (), parent, children)
}

/// Spawns a [`ButtonBundle`] with children.
pub fn buttoni<P>(
    class: impl Class<P, In = ButtonBundle>,
    extras: impl Bundle,
    parent: &mut UiChildBuilder,
    children: impl FnOnce(&mut UiChildBuilder),
) -> Entity {
    let mut bundle = ButtonBundle::default();
    class.apply(&mut bundle, parent.world);
    parent.spawn((bundle, extras)).with_children(children).id()
}

/// Spawns a [`ButtonBundle`] without children.
pub fn simple_button<P>(
    class: impl Class<P, In = ButtonBundle>,
    parent: &mut UiChildBuilder,
) -> Entity {
    simple_buttoni(class, (), parent)
}

/// Spawns a [`ButtonBundle`] without children.
pub fn simple_buttoni<P>(
    class: impl Class<P, In = ButtonBundle>,
    extras: impl Bundle,
    parent: &mut UiChildBuilder,
) -> Entity {
    let mut bundle = ButtonBundle::default();
    class.apply(&mut bundle, parent.world);
    parent.spawn((bundle, extras)).id()
}

/// Spawns a [`ButtonBundle`] with a single [`TextBundle`] as its child.
pub fn text_button<P, P1>(
    txt: impl Into<String>,
    class: impl Class<P, In = ButtonBundle>,
    text_style: impl Class<P1, In = TextStyle>,
    parent: &mut UiChildBuilder,
) -> Entity {
    text_buttoni(txt, class, text_style, (), parent)
}

/// Spawns a [`ButtonBundle`] with a single [`TextBundle`] as its child.
pub fn text_buttoni<P, P1>(
    txt: impl Into<String>,
    class: impl Class<P, In = ButtonBundle>,
    text_style: impl Class<P1, In = TextStyle>,
    extras: impl Bundle,
    parent: &mut UiChildBuilder,
) -> Entity {
    buttoni(class, extras, parent, |p| {
        text(txt, (), text_style, p);
    })
}

/// Spawns an [`ImageBundle`].
pub fn image<P>(class: impl Class<P, In = ImageBundle>, parent: &mut UiChildBuilder) -> Entity {
    imagei(class, (), parent)
}

/// Spawns an [`ImageBundle`].
pub fn imagei<P>(
    class: impl Class<P, In = ImageBundle>,
    extras: impl Bundle,
    parent: &mut UiChildBuilder,
) -> Entity {
    let mut bundle = ImageBundle::default();
    class.apply(&mut bundle, parent.world);
    parent.spawn((bundle, extras)).id()
}

/// Spawns an [`ImageBundle`] with children.
pub fn image_pane<P>(
    class: impl Class<P, In = ImageBundle>,
    parent: &mut UiChildBuilder,
    children: impl FnOnce(&mut UiChildBuilder),
) -> Entity {
    image_panei(class, parent, (), children)
}

/// Spawns an [`ImageBundle`] with children.
pub fn image_panei<P>(
    class: impl Class<P, In = ImageBundle>,
    parent: &mut UiChildBuilder,
    extras: impl Bundle,
    children: impl FnOnce(&mut UiChildBuilder),
) -> Entity {
    let mut bundle = ImageBundle::default();
    class.apply(&mut bundle, parent.world);
    parent.spawn((bundle, extras)).with_children(children).id()
}

/// Spawns a [`NodeBundle`] composed of [`NodeBundle`] cells in the form of a grid.
/// The callback function argument spawns the contents of those cells.
pub fn grid<P>(
    rows: usize,
    columns: usize,
    class: impl Class<P, In = NodeBundle>,
    parent: &mut UiChildBuilder,
    children: impl FnMut(&mut UiChildBuilder, usize, usize),
) -> Entity {
    gridi(rows, columns, class, (), parent, children)
}

/// Spawns a [`NodeBundle`] composed of [`NodeBundle`] cells in the form of a grid.
/// The callback function argument spawns the contents of those cells.
pub fn gridi<P>(
    rows: usize,
    columns: usize,
    class: impl Class<P, In = NodeBundle>,
    extras: impl Bundle,
    parent: &mut UiChildBuilder,
    mut children: impl FnMut(&mut UiChildBuilder, usize, usize),
) -> Entity {
    // Spawns container
    let mut container_bundle = NodeBundle::default();
    class.apply(&mut container_bundle, parent.world);
    container_bundle.style.flex_wrap = FlexWrap::Wrap;
    let mut container = parent.spawn((container_bundle, extras));

    // Spawns cells as children of the container
    let cell_bundle = NodeBundle {
        style: Style {
            width: Val::Percent(100.0 / columns as f32),
            height: Val::Percent(100.0 / rows as f32),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        ..Default::default()
    };
    for row in 0..rows {
        for col in 0..columns {
            container = container.with_children(|container| {
                container
                    .spawn(cell_bundle.clone())
                    .with_children(|cell| children(cell, row, col));
            });
        }
    }
    container.id()
}
