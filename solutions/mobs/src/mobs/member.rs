#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    Underboss,
    Caporegime,
    Soldier,
    Associate,
}

impl Role {
    pub fn power(self) -> u8 {
        match self {
            Role::Underboss  => 4,
            Role::Caporegime => 3,
            Role::Soldier    => 2,
            Role::Associate  => 1,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Member {
    pub role: Role,
    pub age:  u32,
}

impl Member {
    // Promote one step; panic if already Underboss.
    pub fn get_promotion(&mut self) {
        self.role = match self.role {
            Role::Associate  => Role::Soldier,
            Role::Soldier    => Role::Caporegime,
            Role::Caporegime => Role::Underboss,
            Role::Underboss  => panic!("Underboss cannot be promoted further"),
        };
    }
}
