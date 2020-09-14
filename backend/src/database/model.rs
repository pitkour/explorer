#[derive(Queryable)]
pub struct User {
    pub(crate) uuid: String,
    pub(crate) first_nick: String,
    pub(crate) nick: String,
    pub(crate) password: String,
    pub(crate) permission_group: String,
    pub(crate) experience: i32,
    pub(crate) level: i32,
    pub(crate) quest_progress: i32,
    pub(crate) coins: i32,
    pub(crate) spent_coins: i32,
    pub(crate) chests: i32,
    pub(crate) opened_chests: i32,
    pub(crate) won_duels: i32,
    pub(crate) lost_duels: i32,
    pub(crate) first_join_time: f64,
    pub(crate) last_join_time: f64,
    pub(crate) last_quit_time: f64,
    pub(crate) play_time: f64,
}

#[derive(Queryable)]
pub struct Team {
    pub(crate) tag: String,
    pub(crate) name: String,
    pub(crate) creator: String,
    pub(crate) create_time: f64,
    pub(crate) coins: i32,
}

#[derive(Queryable)]
pub struct TeamMember {
    pub(crate) tag: String,
    pub(crate) uuid: String,
    pub(crate) rank: String,
    pub(crate) join_time: f64,
    pub(crate) coins_paid: i32,
}

#[derive(Queryable)]
pub struct Course {
    pub(crate) course_id: i32,
    pub(crate) experience: i32,
    pub(crate) required_level: i32,
    pub(crate) name: String,
    pub(crate) start: String,
    pub(crate) category: String,
}

#[derive(Queryable)]
pub struct Quest {
    pub(crate) quest_id: i32,
    pub(crate) course_id: i32,
    pub(crate) npc_uuid: String,
    pub(crate) npc_nick: String,
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) normal_dialog: String,
    pub(crate) tip_dialog: String,
    pub(crate) start_dialog: String,
    pub(crate) end_dialog: String,
}

#[derive(Queryable)]
pub struct PermanentBan {
    pub(crate) uuid: String,
    pub(crate) nick: String,
    pub(crate) reason: String,
    pub(crate) performer: String,
    pub(crate) create_time: f64,
}
