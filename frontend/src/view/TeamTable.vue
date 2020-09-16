<template>
    <v-container>
        <v-data-table
            :headers="headers"
            :items="teams"
            :items-per-page="10"
            :footer-props="{ 'items-per-page-options': [5, 10, 15, 20, -1] }"
            class="my-10 elevation-2"
        ></v-data-table>
    </v-container>
</template>

<script>
import Queries from "../api/queries";

export default {
    name: "TeamTable",
    apollo: {
        teams: {
            query: Queries.getTeams,
            variables: {
                items: 50
            }
        }
    },
    data() {
        return {
            teams: [],
            headers: [
                {
                    text: "Tag",
                    value: "tag"
                },
                {
                    text: "Name",
                    value: "name"
                },
                {
                    text: "Creator",
                    value: "creator"
                },
                {
                    text: "Create Time",
                    value: "createTime"
                },
                {
                    text: "Coins",
                    value: "coins"
                }
            ]
        };
    }
};
</script>
