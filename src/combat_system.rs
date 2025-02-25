use super::prelude::*;
mod death;
mod melee;
mod ranged;

// Make Plugin
pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(
            Update,
            (
            melee::combat_system_melee
            .run_if(bevy::time::common_conditions::on_timer(bevy::utils::Duration::from_secs_f32(0.5)))
            .run_if(in_state(GameState::InGame))
            ,
            ranged::combat_system_ranged
            .run_if(bevy::time::common_conditions::on_timer(bevy::utils::Duration::from_secs_f32(0.5)))
            .run_if(in_state(GameState::InGame))
            ,
            melee::attacked_entities_system
            .run_if(in_state(GameState::InGame)),
            melee::temporary_visual_elements_system
            .run_if(in_state(GameState::InGame)),
            )
        )
        .add_systems(
            Update,
            death::death_system
        )
        // .add_system(
        //     ranged::combat_system_ranged
        //     .run_if(bevy::time::common_conditions::on_timer(bevy::utils::Duration::from_secs_f32(0.5)))
        //     .run_if(in_state(GameState::InGame))
        // )
        // .add_system(
        //     melee::attacked_entities_system
        //     .run_if(in_state(GameState::InGame))
        // )
        // .add_fixed_timestep_system(
        //     HALF_SECOND,
        //     0,
        //     melee::combat_system_melee.run_in_bevy_state(GameState::InGame),
        // )
        // .add_fixed_timestep_system(
        //     HALF_SECOND,
        //     0,
        //     ranged::combat_system_ranged.run_in_bevy_state(GameState::InGame),
        // )
        // .add_system(
        //     melee::attacked_entities_system
        //     .run_in_bevy_state(GameState::InGame)
            
        // )
        ;
    }
}
