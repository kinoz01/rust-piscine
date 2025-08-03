
pub mod boss;
pub mod member;

pub use boss::Boss;
pub use member::{Member, Role};

use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq)]
pub struct Mob {
    pub name:     String,
    pub boss:     Boss,
    pub members:  HashMap<String, Member>,
    pub cities:   HashSet<String>,
    pub wealth:   u64,
}

impl Mob {
    fn power_score(&self) -> u64 {
        self.members.values().map(|m| m.role.power() as u64).sum()
    }

    pub fn new(
        name: String,
        boss: Boss,
        members: HashMap<String, Member>,
        cities: HashSet<String>,
        wealth: u64,
    ) -> Self {
        Self {
            name: name,
            boss,
            members,
            cities,
            wealth,
        }
    }

    pub fn recruit(&mut self, (name, age): (&str, u32)) {
        self.members.insert(
            name.into(),
            Member {
                role: Role::Associate,
                age,
            },
        );
    }

    pub fn attack(&mut self, other: &mut Mob) {
        let (self_power, other_power) = (self.power_score(), other.power_score());

        let (winner, loser) = if self_power > other_power {
            (self, other)
        } else if self_power < other_power {
            (other, self)
        } else {
            (other, self)
        };

        if let Some(min_age) = loser.members.values().map(|m| m.age).min() {
            let to_remove: Vec<String> = loser
                .members
                .iter()
                .filter(|(_, m)| m.age == min_age)
                .map(|(n, _)| n.clone())
                .collect();
            for n in to_remove {
                loser.members.remove(&n);
            }
        }

        if loser.members.is_empty() {
            winner.cities.extend(loser.cities.drain());
            winner.wealth += loser.wealth;
            loser.wealth = 0;
        }
    }

    pub fn steal(&mut self, target: &mut Mob, amount: u64) {
        let take = amount.min(target.wealth);
        target.wealth -= take;
        self.wealth += take;
    }

    pub fn conquer_city(&mut self, others: &[&Mob], city: String) {
        if self.cities.contains(&city) {
            return;
        }
        if others.iter().any(|m| m.cities.contains(&city)) {
            return;
        }
        self.cities.insert(city);
    }
}
