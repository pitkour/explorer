<template>
    <v-container>
        <v-card class="elevation-2">
            <v-card-title>
                <v-row>
                    <v-icon class="ml-4 mr-2">mdi-gavel</v-icon>
                    Permanent Bans
                </v-row>

                <v-spacer></v-spacer>

                <v-text-field
                    v-model="searchQuery"
                    append-icon="mdi-magnify"
                    label="Search"
                    single-line
                    hide-details
                ></v-text-field>
            </v-card-title>

            <v-data-table
                :headers="headers"
                :items="permanentBans"
                :items-per-page="itemsPerPage"
                :footer-props="{ itemsPerPageOptions: [5, 10, 15, 20, -1] }"
            >
                <template v-slot:[`item.controls.view`]="{ item }">
                    <v-btn x-small fab :to="'/permanent-ban/' + item.uuid">
                        <v-icon>mdi-eye</v-icon>
                    </v-btn>
                </template>

                <template v-slot:[`item.createTime`]="{ item }">
                    {{ formatUnixTimestamp(item.createTime) }}
                </template>
            </v-data-table>
        </v-card>
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
                    items: this.itemsPerPage * 3,
                    searchQuery: this.searchQuery
                };
            }
        }
    },

    methods: {
        formatUnixTimestamp(timestamp) {
            return FormatUtil.formatUnixTimestamp(timestamp);
        }
    },

    watch: {
        searchQuery(value) {
            this.searchQuery = value ? value : null;
        }
    },

    data: () => ({
        itemsPerPage: 10,
        searchQuery: null,
        permanentBans: [],
        headers: [
            {
                text: "",
                value: "controls.view",
                sortable: false
            },
            {
                text: "UUID",
                value: "uuid"
            },
            {
                text: "Nick",
                value: "nick"
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
