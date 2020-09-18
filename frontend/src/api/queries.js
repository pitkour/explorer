import gql from "graphql-tag";

export default {
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
