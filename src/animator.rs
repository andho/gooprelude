use std::{fmt::Display, hash::Hash};

use bevy::{
    core::Name,
    hierarchy::Children,
    prelude::{info, Commands, Component, Entity, Handle, Query, With},
    sprite::TextureAtlasSprite,
    utils::HashMap,
};

use crate::animation::SpriteSheetAnimation;

pub trait AnimationKey: Eq + Hash + Sync + Send + Default + Display {}

#[derive(Component)]
pub struct Animator<T: AnimationKey, U> {
    animations: HashMap<T, Handle<SpriteSheetAnimation>>,
    selector: fn(U) -> T,
    target: Name,
}

impl<T: AnimationKey, U> Animator<T, U> {
    pub fn new(
        animations: HashMap<T, Handle<SpriteSheetAnimation>>,
        selector: fn(U) -> T,
        target: Name,
    ) -> Self {
        Self {
            animations,
            selector,
            target,
        }
    }

    pub fn select(&self, data: U) -> Handle<SpriteSheetAnimation> {
        let animation: T = (self.selector)(data);

        self.animations.get(&animation).unwrap().clone_weak()
    }

    pub fn match_target(&self, name: &Name) -> bool {
        self.target == *name
    }
}

pub fn animation_selection<T: AnimationKey + 'static, U: 'static + Component + Clone>(
    mut commands: Commands,
    animated: Query<(Entity, &Animator<T, U>, &U, &Children)>,
    sprites: Query<&Name, With<TextureAtlasSprite>>,
) {
    for (entity, animator, anim_data, children) in animated.iter() {
        let animation = animator.select(anim_data.clone());
        if let Ok(name) = sprites.get(entity) {
            if animator.match_target(name) {
                commands.entity(entity).insert(animation);
                continue;
            }
        }

        let child_entity = children.iter().find(|child| {
            if let Ok(name) = sprites.get(**child) {
                if animator.match_target(name) {
                    return true;
                }
            }
            false
        });

        if let Some(child_entity) = child_entity {
            commands.entity(*child_entity).insert(animation);
        }
    }
}
