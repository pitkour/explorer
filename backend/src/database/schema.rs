table! {
    pitkour_courses (courseID) {
        courseID -> Integer,
        experience -> Integer,
        requiredLevel -> Integer,
        name -> Text,
        start -> Text,
        category -> Text,
    }
}

table! {
    pitkour_permamentbans (uuid) {
        uuid -> Text,
        nick -> Text,
        reason -> Text,
        performer -> Text,
        createTime -> Double,
    }
}

table! {
    pitkour_quests (questID) {
        questID -> Integer,
        courseID -> Integer,
        npcUUID -> Text,
        npcNick -> Text,
        name -> Text,
        description -> Text,
        normalDialog -> Text,
        tipDialog -> Text,
        startDialog -> Text,
        endDialog -> Text,
    }
}

table! {
    pitkour_teams (tag) {
        tag -> Text,
        name -> Text,
        creator -> Text,
        createTime -> Double,
        coins -> Integer,
    }
}

table! {
    pitkour_teams_members (uuid) {
        tag -> Text,
        uuid -> Text,
        rank -> Text,
        joinTime -> Double,
        coinsPaid -> Integer,
    }
}

table! {
    pitkour_users (uuid) {
        uuid -> Text,
        firstNick -> Text,
        nick -> Text,
        password -> Text,
        permissionGroup -> Text,
        experience -> Integer,
        level -> Integer,
        questProgress -> Integer,
        coins -> Integer,
        spentCoins -> Integer,
        chests -> Integer,
        openedChests -> Integer,
        wonDuels -> Integer,
        lostDuels -> Integer,
        firstJoinTime -> Double,
        lastJoinTime -> Double,
        lastQuitTime -> Double,
        playTime -> Double,
    }
}

allow_tables_to_appear_in_same_query!(
    pitkour_courses,
    pitkour_permamentbans,
    pitkour_quests,
    pitkour_teams,
    pitkour_teams_members,
    pitkour_users,
);
