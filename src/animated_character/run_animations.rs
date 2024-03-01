use bevy::prelude::*;

use super::{link_animations::AnimationEntityLink, Animations, PlayerCharacterName};

pub fn run_animations(
    mut animation_player_query: Query<&mut AnimationPlayer>,
    mut player_character_query: Query<
        (&PlayerCharacterName, &AnimationEntityLink),
        Added<AnimationEntityLink>,
    >,
    animations: Res<Animations>,
) {
    for (player_character_name, animation_entity_link) in &mut player_character_query.iter_mut() {
        if let Ok(mut animation_player) = animation_player_query.get_mut(animation_entity_link.0) {
            if player_character_name.0 == "Player 2".to_string() {
                animation_player
                    .play(
                        animations
                            .0
                            .get("Death")
                            .expect("animation to exist")
                            .clone_weak(),
                    )
                    .repeat()
                    .set_speed(0.5);
            }

            if player_character_name.0 == "Player 1".to_string() {
                animation_player
                    .play(
                        animations
                            .0
                            .get("Idle")
                            .expect("animation to exist")
                            .clone_weak(),
                    )
                    .repeat()
                    .set_speed(0.5);
            }
        }
    }
}