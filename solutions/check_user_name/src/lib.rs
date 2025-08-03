pub enum AccessLevel  {
    Guest, 
    Normal, 
    Admin,
}

pub struct User {
    name: String,
    level: AccessLevel,
}

impl User {
    pub fn new(name: String, level: AccessLevel) -> Self {
        Self {name, level }
    }
    pub fn send_name(&self) -> Option<&str> {
        match self.level {
            AccessLevel::Guest => None,
            _ => Some(&self.name)
        }
    }
}

pub fn check_user_name(user: &User) -> (bool, &str) {
    match user.send_name() {
        None => (false, "ERROR: User is guest"),
        Some(name) => (true, name), 
    }
}