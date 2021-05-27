use rand::{
    distributions::{Distribution, WeightedIndex},
    Rng,
};
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;
use crate::utils::decay_calculator;

// Generate a random variable reward to include in new blocks
pub const TOTAL_NUGGETS: u32 = 80000000;
pub const TOTAL_VEINS: u32 = 1400000;
pub const TOTAL_MOTHERLODES: u32 = 20000;
pub const N_BLOCKS_PER_EPOCH: u32 = 16000000;
pub const NUGGET_FINAL_EPOCH: u16 = 300;
pub const VEIN_FINAL_EPOCH: u8 = 200;
pub const MOTHERLODE_FINAL_EPOCH: u8 = 100;
pub const FLAKE_REWARD_RANGE: (u32, u32) = (8u32.pow(0), 8u32.pow(1) - 1u32);
pub const GRAIN_REWARD_RANGE: (u32, u32) = (8u32.pow(1), 8u32.pow(2) - 1u32);
pub const NUGGET_REWARD_RANGE: (u32, u32) = (8u32.pow(2), 8u32.pow(3) - 1u32);
pub const VEIN_REWARD_RANGE: (u32, u32) = (8u32.pow(3), 8u32.pow(4) - 1u32);
pub const MOTHERLODE_REWARD_RANGE: (u32, u32) = (8u32.pow(4), 8u32.pow(5) - 1u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, EnumIter)]
pub enum Category {
    Flake(Option<i128>),
    Grain(Option<i128>),
    Nugget(Option<i128>),
    Vein(Option<i128>),
    Motherlode(Option<i128>),
}

#[allow(dead_code)]
pub struct RewardState {
    pub epoch: u32,
    pub next_epoch_block: u32,
    pub current_block: u32,
    pub n_nuggets_remaining: u32,
    pub n_veins_remaining: u32,
    pub n_motherlodes_remaining: u32,
    pub n_nuggets_current_epoch: u32,
    pub n_veins_current_epoch: u32,
    pub n_motherlodes_current_epoch: u32,
    pub n_flakes_current_epoch: u32,
    pub n_grains_current_epoch: u32,
}

#[allow(dead_code)]
pub struct Reward {
    pub miner: String,
    pub category: Category,
    pub amount: i128,
}

impl RewardState {
    pub fn start() -> RewardState {
        let n_nuggets_ce: u32 = (decay_calculator(
            TOTAL_NUGGETS, NUGGET_FINAL_EPOCH as u32) * 
            TOTAL_NUGGETS as f64) as u32;
        let n_veins_ce: u32 = (decay_calculator(
            TOTAL_NUGGETS, NUGGET_FINAL_EPOCH as u32) * 
            TOTAL_NUGGETS as f64) as u32;
        let n_motherlodes_ce: u32 = (decay_calculator(
            TOTAL_NUGGETS, NUGGET_FINAL_EPOCH as u32) * 
            TOTAL_NUGGETS as f64) as u32;
        let remaining_blocks = N_BLOCKS_PER_EPOCH - (n_nuggets_ce + n_veins_ce + n_motherlodes_ce);
        let n_flakes_ce: u32 = (remaining_blocks as f64 * 0.6f64) as u32;
        let n_grains_ce: u32 = (remaining_blocks as f64 * 0.4f64) as u32;

        RewardState {
            current_block: 0,
            epoch: 1,
            next_epoch_block: 16000000,
            n_nuggets_remaining: TOTAL_NUGGETS,
            n_veins_remaining: TOTAL_VEINS,
            n_motherlodes_remaining: TOTAL_MOTHERLODES,
            n_nuggets_current_epoch: n_nuggets_ce,
            n_veins_current_epoch: n_veins_ce,
            n_motherlodes_current_epoch: n_motherlodes_ce,
            n_flakes_current_epoch: n_flakes_ce,
            n_grains_current_epoch: n_grains_ce,
            
        }
    }
    pub fn update(&self, last_reward: Category) -> Self {
        let mut n_nuggets_ce: u32 = self.n_nuggets_current_epoch;
        let mut n_veins_ce: u32 = self.n_veins_current_epoch;
        let mut n_motherlodes_ce: u32 = self.n_motherlodes_current_epoch;
        let mut n_flakes_ce: u32 = self.n_flakes_current_epoch;
        let mut n_grains_ce: u32 = self.n_grains_current_epoch;
        let remaining_blocks_in_ce: u32 = self.next_epoch_block - (self.current_block + 1);
        if remaining_blocks_in_ce != 0 {
            n_nuggets_ce = match last_reward {
                Category::Nugget(Some(_)) => n_nuggets_ce - 1,
                _ => n_nuggets_ce,
                };
            n_veins_ce = match last_reward {
                Category::Vein(Some(_)) => n_veins_ce - 1,
                _ => n_veins_ce,
                };
            n_motherlodes_ce = match last_reward {
                Category::Motherlode(Some(_)) => n_motherlodes_ce - 1,
                _ => n_motherlodes_ce,
                };
            n_flakes_ce = match last_reward {
                Category::Flake(Some(_)) => n_flakes_ce - 1,
                _ => n_flakes_ce,
                };
            n_grains_ce = match last_reward {
                Category::Grain(Some(_)) => n_grains_ce -1,
                _ => n_grains_ce,
            };
        } else {
            n_nuggets_ce = (decay_calculator(
            TOTAL_NUGGETS, NUGGET_FINAL_EPOCH as u32) * 
            self.n_nuggets_remaining as f64) as u32;
            n_veins_ce = (decay_calculator(
                TOTAL_NUGGETS, NUGGET_FINAL_EPOCH as u32) * 
                self.n_veins_remaining as f64) as u32;
            n_motherlodes_ce = (decay_calculator(
                TOTAL_NUGGETS, NUGGET_FINAL_EPOCH as u32) * 
                self.n_motherlodes_remaining as f64) as u32;
            let remaining_blocks = N_BLOCKS_PER_EPOCH - (n_nuggets_ce + n_veins_ce + n_motherlodes_ce);
            n_flakes_ce = (remaining_blocks as f64 * 0.6f64) as u32;
            n_grains_ce = (remaining_blocks as f64 * 0.4f64) as u32;
        }
        Self {
            current_block: self.current_block + 1,
            epoch: if self.current_block + 1 != self.next_epoch_block {
                self.epoch
            } else {
                self.epoch + 1
            },
            next_epoch_block: if self.current_block + 1 != self.next_epoch_block {
                self.next_epoch_block
            } else {
                self.next_epoch_block + N_BLOCKS_PER_EPOCH
            },
            n_nuggets_remaining: match last_reward {
                Category::Nugget(Some(_)) => self.n_nuggets_remaining - 1,
                _ => self.n_nuggets_remaining,
            },
            n_veins_remaining: match last_reward {
                Category::Vein(Some(_)) => self.n_veins_remaining - 1,
                _ => self.n_veins_remaining,
            },
            n_motherlodes_remaining: match last_reward {
                Category::Motherlode(Some(_)) => self.n_motherlodes_remaining - 1,
                _ => self.n_motherlodes_remaining,
            },
            n_nuggets_current_epoch: n_nuggets_ce,
            n_veins_current_epoch: n_veins_ce,
            n_motherlodes_current_epoch: n_motherlodes_ce,
            n_flakes_current_epoch: n_flakes_ce,
            n_grains_current_epoch: n_grains_ce,
            }
        }
    }

impl Reward {
    pub fn new(miner: String, reward_state: &RewardState) -> Reward {
        let category: Category = Category::new(reward_state);
        Reward {
            miner: miner,
            category: category,
            amount: match category {
                Category::Flake(Some(amount)) => amount,
                Category::Grain(Some(amount)) => amount,
                Category::Nugget(Some(amount)) => amount,
                Category::Vein(Some(amount)) => amount,
                Category::Motherlode(Some(amount)) => amount,
                _ => 0,             // Add error handling, as this should NEVER happen.
            },
        }
    }
}

impl Category {
    pub fn new(reward_state: &RewardState) -> Category {
        Category::generate_category(reward_state).amount()
    }

    pub fn generate_category(reward_state: &RewardState) -> Category {
        let items = vec![
            (Category::Flake(None), reward_state.n_flakes_current_epoch),
            (Category::Grain(None), reward_state.n_grains_current_epoch),
            (Category::Nugget(None), reward_state.n_nuggets_current_epoch),
            (Category::Vein(None), reward_state.n_veins_current_epoch),
            (Category::Motherlode(None), reward_state.n_veins_current_epoch),
            ];
        let dist = WeightedIndex::new(items.iter().map(|item| item.1)).unwrap();
        let mut rng = rand::thread_rng();
        let category = items[dist.sample(&mut rng)].0;
        category
    }

    pub fn amount(&self) -> Category {
        match self {
            Self::Flake(None) => Category::Flake(Some(
                rand::thread_rng()
                    .gen_range(FLAKE_REWARD_RANGE.0, FLAKE_REWARD_RANGE.1)
                    .into(),
            )),
            Self::Grain(None) => Category::Flake(Some(
                rand::thread_rng()
                    .gen_range(GRAIN_REWARD_RANGE.0, GRAIN_REWARD_RANGE.1)
                    .into(),
            )),
            Self::Nugget(None) => Category::Nugget(Some(
                rand::thread_rng()
                    .gen_range(NUGGET_REWARD_RANGE.0, NUGGET_REWARD_RANGE.1)
                    .into(),
            )),
            Self::Vein(None) => Category::Vein(Some(
                rand::thread_rng()
                    .gen_range(VEIN_REWARD_RANGE.0, VEIN_REWARD_RANGE.1)
                    .into(),
            )),
            Self::Motherlode(None) => Category::Motherlode(Some(
                rand::thread_rng()
                    .gen_range(MOTHERLODE_REWARD_RANGE.0, MOTHERLODE_REWARD_RANGE.1)
                    .into(),
            )),
            _ => Category::Flake(Some(rand::thread_rng()
                                        .gen_range(FLAKE_REWARD_RANGE.0, FLAKE_REWARD_RANGE.1)
                                        .into()
            ))
        }
    }
}