<template>
    <v-container>
        <v-data-table
            :headers="headers"
            :items="users"
            :items-per-page="10"
            :footer-props="{ 'items-per-page-options': [5, 10, 15, 20, -1] }"
            class="my-10 elevation-2"
        >
            <template v-slot:[`item.firstLogin`]="{ item }">
                {{ formatUnixTimestamp(item.firstLogin) }}
            </template>
        </v-data-table>
    </v-container>
</template>

<script>
import Queries from "../api/queries";
import DateUtil from "../util/date-util";

export default {
    name: "UserTable",
    apollo: {
        users: {
            query: Queries.getUsers,
            variables() {
                return {
                    items: this.itemsPerPage * 3
                };
            }
        }
    },
    methods: {
        formatUnixTimestamp(timestamp) {
            return DateUtil.formatUnixTimestamp(timestamp);
        }
    },
    data: () => ({
        itemsPerPage: 10,
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
            },
            {
                text: "First Login",
                value: "firstLogin"
            }
        ]
    })
};
</script>
