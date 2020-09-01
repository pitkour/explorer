#[derive(GraphQLObject)]
pub struct Team {
    pub tag: String,
    pub name: String,
    pub creator: String,
    pub create_time: f64,
    pub coins: i32,
}
