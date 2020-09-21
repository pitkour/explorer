import gql from "graphql-tag";

export default {
    updateUser: gql`
        mutation updateUser(
            $uuid: String!
            $firstNick: String
            $nick: String
            $experience: Int
            $level: Int
        ) {
            updateUser(
                input: {
                    uuid: $uuid
                    firstNick: $firstNick
                    nick: $nick
                    experience: $experience
                    level: $level
                }
            ) {
                affectedRows
            }
        }
    `,

    updateTeam: gql`
        mutation updateTeam(
            $tag: String!
            $name: String
            $coins: Int
            $creator: String
            $createTime: Float
        ) {
            updateTeam(
                input: {
                    tag: $tag
                    name: $name
                    coins: $coins
                    creator: $creator
                    createTime: $createTime
                }
            ) {
                affectedRows
            }
        }
    `,

    updatePermanentBan: gql`
        mutation updatePermanentBan(
            $uuid: String!
            $nick: String
            $reason: String
            $performer: String
            $createTime: Float
        ) {
            updatePermanentBan(
                input: {
                    uuid: $uuid
                    nick: $nick
                    reason: $reason
                    performer: $performer
                    createTime: $createTime
                }
            ) {
                affectedRows
            }
        }
    `
};
