import gql from "graphql-tag";

export default {
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
    `
};
