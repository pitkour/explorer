#[derive(Queryable, Debug)]
pub struct Team {
    pub(crate) tag: String,
    pub(crate) name: String,
    pub(crate) creator: String,
    pub(crate) create_time: f64,
    pub(crate) coins: i32,
}

#[derive(Queryable, Debug)]
pub struct TeamMember {
    pub(crate) tag: String,
    pub(crate) uuid: String,
    pub(crate) rank: String,
    pub(crate) join_time: f64,
    pub(crate) coins_paid: i32,
}
