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
                firstLogin: firstJoinTime
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
                permanentBan {
                    uuid
                }
            }
        }
    `,

    getUsers: gql`
        query getUsers($items: Int!, $searchQuery: String) {
            users(first: $items, searchQuery: $searchQuery) {
                nick
                uuid
                level
                experience
                firstLogin: firstJoinTime
            }
        }
    `,

    getTeam: gql`
        query getTeam($tag: String!) {
            team(tag: $tag) {
                tag
                name
                creator
                createTime
                coins
                members {
                    coinsPaid
                    joinTime
                    rank
                    user {
                        nick
                        uuid
                    }
                }
            }
        }
    `,

    getTeams: gql`
        query getTeams($items: Int!, $searchQuery: String) {
            teams(first: $items, searchQuery: $searchQuery) {
                tag
                name
                creator
                createTime
                coins
                membersCount
            }
        }
    `,

    getPermanentBan: gql`
        query getPermanentBan($uuid: String!) {
            permanentBan(uuid: $uuid) {
                nick
                uuid
                performer
                createTime
                reason
            }
        }
    `,

    getPermanentBans: gql`
        query getPermanentBans($items: Int!, $searchQuery: String) {
            permanentBans(first: $items, searchQuery: $searchQuery) {
                nick
                uuid
                performer
                createTime
                reason
            }
        }
    `
};
