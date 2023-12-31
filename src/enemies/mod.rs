use crate::battle_logic::{Battlable, BattleAction, BattleState};
use std::sync::Arc;
// use serde::{Deserialize, Serialize};

pub mod boss;

pub enum Lifeness {
    Alive,
    Dead,
}

pub trait Enemy: Battlable {
    // TODO: write enemy trait

    // fn new() -> Self
    // where
    //     Self: Sized;

    /// returns the enemies name. (eg, "Orc", "Goblin #1", "Gregory the Destroyer of Worlds", etc)
    fn get_name(&self) -> Arc<str>;

    /// generates the move that the enemy will take
    fn get_move(&mut self, state: &mut BattleState) -> BattleAction;

    /// applies damage to the Enemy
    fn take_damage(&mut self, state: &mut BattleState) -> Lifeness;
}

// #[derive(Serialize, Deserialize, PartialEq, Eq, Hash)]
// pub struct EnemyKey {
//     level: u8,
//     loc: (u8, u8),
//     name: Arc<str>,
//     maker_adr: I2cAdr,
// }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MobSpawner {}

impl MobSpawner {}
