<template>
    <v-container>
        <v-data-table
            :headers="headers"
            :items="users"
            :items-per-page="10"
            :footer-props="{ 'items-per-page-options': [5, 10, 20, -1] }"
            class="my-10 elevation-2"
        ></v-data-table>
    </v-container>
</template>

<script>
import Queries from "../api/queries";

export default {
    apollo: {
        users: {
            query: Queries.getUsers,
            variables: {
                items: 50
            }
        }
    },
    name: "UserTable",
    data() {
        return {
            users: [],
            headers: [
                {
                    text: "UUID",
                    value: "uuid"
                },
                {
                    text: "Nick",
                    value: "nick"
                },
                {
                    text: "Level",
                    value: "level"
                },
                {
                    text: "Experience",
                    value: "experience"
                }
            ]
        };
    }
};
</script>
