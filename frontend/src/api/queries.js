import gql from "graphql-tag";

export default {
    getUsers: gql`
        query getUsers($items: Int!) {
            users(first: $items) {
                nick
                uuid
                level
                experience
            }
        }
    `
};
