<template>
    <v-container>
        <v-data-table
            :headers="headers"
            :items="permanentBans"
            :items-per-page="10"
            :footer-props="{ 'items-per-page-options': [5, 10, 15, 20, -1] }"
            class="my-10 elevation-2"
        ></v-data-table>
    </v-container>
</template>

<script>
import Queries from "../api/queries";

export default {
    name: "PermanentBanTable",
    apollo: {
        permanentBans: {
            query: Queries.getPermanentBans,
            variables: {
                items: 50
            }
        }
    },
    data() {
        return {
            permanentBans: [],
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
                    text: "Reason",
                    value: "reason"
                },
                {
                    text: "Performer",
                    value: "performer"
                },
                {
                    text: "Created",
                    value: "createTime"
                }
            ]
        };
    }
};
</script>
