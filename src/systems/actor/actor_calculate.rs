// pub fn actor_calculate_hit_chance(

// ) {
//     if let Some((attacker_entity, attacker)) = &ctx.attacker {
//         for (defender_entity, defender) in &ctx.defenders {
//             if chance_engine.roll_hit_chance(attacker, defender) {
//                 let damage = chance_engine.roll_damage_chance();

//                 if defender.health > 0 {
//                     event_handler.emit(OnHitEvent {
//                         attacker: attacker_entity,
//                         defender: defender_entity,
//                         damage_dealt: damage as f32,
//                     });
//                 } else {
//                     event_handler.emit(OnDeathEvent {
//                         attacker: *attacker_entity,
//                         defender: *defender_entity,
//                         damage_dealt: damage as f32,
//                     });
//                 }
//             }
//         }
//     }
// }