// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    fn new(level: u32) -> Player {
        Self {
            health: 100,
            mana: if level < 10 { None } else { Some(100) },
            level,
        }
    }

    pub fn revive(&self) -> Option<Player> {
        match self.health {
            0 => Some(Player::new(self.level)),
            _ => None,
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            Some(mana) => {
                if mana >= mana_cost {
                    self.mana = Some(mana - mana_cost);
                    return mana_cost * 2;
                } else {
                    return 0;
                }
            }
            None => {
                if mana_cost >= self.health {
                    self.health = 0;
                } else {
                    self.health -= mana_cost;
                }
                return 0;
            }
        }
    }
}
