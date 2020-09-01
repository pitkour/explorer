#[derive(Queryable, Debug)]
pub struct Team {
    pub tag: String,
    pub name: String,
    pub creator: String,
    pub create_time: f64,
    pub coins: i32,
}

#[derive(Queryable, Debug)]
pub struct TeamMember {
    pub tag: String,
    pub uuid: String,
    pub rank: String,
    pub join_time: f64,
    pub coins_paid: i32,
}
