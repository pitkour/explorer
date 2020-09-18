<template>
    <v-container>
        <v-data-table
            :headers="headers"
            :items="teams"
            :items-per-page="itemsPerPage"
            :footer-props="{ 'items-per-page-options': [5, 10, 15, 20, -1] }"
            class="my-10 elevation-2"
        >
            <template v-slot:[`item.createTime`]="{ item }">
                {{ formatUnixTimestamp(item.createTime) }}
            </template>
        </v-data-table>
    </v-container>
</template>

<script>
import Queries from "../api/queries";
import DateUtil from "../util/date-util";

export default {
    name: "TeamTable",
    apollo: {
        teams: {
            query: Queries.getTeams,
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
    })
};
</script>
