use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct AnimationEntityLink(pub Entity);

pub fn get_top_parent(mut curr_entity: Entity, query: &Query<&Parent>) -> Entity {
    //Loop up all the way to the top parent
    loop {
        if let Ok(parent) = query.get(curr_entity) {
            curr_entity = parent.get();
        } else {
            break;
        }
    }
    curr_entity
}

pub fn link_animations(
    player_query: Query<Entity, Added<AnimationPlayer>>,
    parent_query: Query<&Parent>,
    animations_entity_link_query: Query<&AnimationEntityLink>,
    mut commands: Commands,
) {
    // Get all the Animation players which can be deep and hidden in the heirachy
    for entity_with_animation_player in player_query.iter() {
        let top_entity = get_top_parent(entity_with_animation_player, &parent_query);

        // If the top parent has an animation config ref then link the player to the config
        if animations_entity_link_query.get(top_entity).is_ok() {
            warn!("Problem with multiple animationsplayers for the same top parent");
        } else {
            println!(
                "linking entity {:#?} to animation_player entity {:#?}",
                top_entity, entity_with_animation_player
            );
            commands
                .entity(top_entity)
                .insert(AnimationEntityLink(entity_with_animation_player.clone()));
        }
    }
}
