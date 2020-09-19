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
        query getTeams($items: Int!) {
            teams(first: $items) {
                tag
                name
                creator
                createTime
                coins
                membersCount
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
