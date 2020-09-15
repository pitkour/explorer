import gql from 'graphql-tag'

export default {
    getUserQuery: gql `
query getUserQuery($items: Int!) {
    userQuery: user {
        users(first: $items) {
            nick
            uuid
            level
            experience
        }
    }
}`

}
