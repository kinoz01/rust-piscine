#[derive(Debug, PartialEq, Eq)]
pub struct OfficeWorker {
    pub name: String,
    pub age: u32,
    pub role: WorkerRole,
}

#[derive(Debug, PartialEq, Eq)]
pub enum WorkerRole {
    Admin,
    User,
    Guest,
}

impl From<&str> for OfficeWorker {
    fn from(s: &str) -> Self {
        let mut cc = s.splitn(3, ',');
        let name = cc.next().unwrap().to_string();
        let age = cc.next().unwrap().parse().unwrap();
        let role = WorkerRole::from(cc.next().unwrap());
        Self {name, age, role}
    }
}

impl From<&str> for WorkerRole {
    fn from(s: &str) -> Self {
        match s {
            "admin" => WorkerRole::Admin,
            "user" => WorkerRole::User,
            "guest" => WorkerRole::Guest,
            _ => unreachable!(),
        }
    }
}