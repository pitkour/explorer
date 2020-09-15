const USERS = [{
        uuid: "0000-00000000-0000-0000",
        nick: "PitceR",
        level: 1,
        experience: 0
    },
    {
        uuid: "0000-00000000-0000-0000",
        nick: "niedoczekanie111",
        level: 1,
        experience: 0
    },
    {
        uuid: "0000-00000000-0000-0000",
        nick: "jaqobb",
        level: 1,
        experience: 0
    },
]

export default {
    getUsers(callback) {
        setTimeout(() => callback(USERS), 100);
    }
}
