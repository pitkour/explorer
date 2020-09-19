import gql from "graphql-tag";

export default {
    getUser: gql`
        query getUser($uuid: String!) {
            user(uuid: $uuid) {
                nick
                uuid
                level
                experience
                chests
                coins
                firstNick
                lastLogin: lastJoinTime
                lastLogout: lastQuitTime
                lostDuels
                openedChests
                permissionGroup
                playTime
                questProgress
                spentCoins
                wonDuels
                teamMember {
                    tag
                }
                firstLogin: firstJoinTime
            }
        }
    `,

    getUsers: gql`
        query getUsers($items: Int!) {
            users(first: $items) {
                nick
                uuid
                level
                experience
                firstLogin: firstJoinTime
            }
        }
    `,
    getTeams: gql`
        query getTeams($items: Int!) {
            teams(first: $items) {
                tag
                name
                creator
                createTime
                coins
            }
        }
    `,
    getPermanentBans: gql`
        query getPermanentBans($items: Int!) {
            permanentBans(first: $items) {
                nick
                uuid
                performer
                createTime
                reason
            }
        }
    `
};
