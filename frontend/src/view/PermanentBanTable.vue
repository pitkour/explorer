<template>
    <v-container>
        <v-data-table
            :headers="headers"
            :items="permanentBans"
            :items-per-page="itemsPerPage"
            :footer-props="{ itemsPerPageOptions: [5, 10, 15, 20, -1] }"
            class="elevation-2"
        >
            <template v-slot:[`item.createTime`]="{ item }">
                {{ formatUnixTimestamp(item.createTime) }}
            </template>
        </v-data-table>
    </v-container>
</template>

<script>
import Queries from "../api/queries";
import FormatUtil from "../util/format-util";

export default {
    name: "PermanentBanTable",

    apollo: {
        permanentBans: {
            query: Queries.getPermanentBans,
            variables() {
                return {
                    items: this.itemsPerPage * 3
                };
            }
        }
    },

    methods: {
        formatUnixTimestamp(timestamp) {
            return FormatUtil.formatUnixTimestamp(timestamp);
        }
    },

    data: () => ({
        itemsPerPage: 10,
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
    })
};
</script>
