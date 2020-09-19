<template>
    <v-container>
        <v-card class="elevation-2">
            <v-card-title>
                <v-row>
                    <v-icon class="ml-4 mr-2">mdi-castle</v-icon>
                    Teams
                </v-row>

                <v-spacer></v-spacer>

                <v-text-field
                    v-model="search"
                    append-icon="mdi-magnify"
                    label="Search"
                    single-line
                    hide-details
                ></v-text-field>
            </v-card-title>

            <v-data-table
                :headers="headers"
                :items="teams"
                :items-per-page="itemsPerPage"
                :footer-props="{ itemsPerPageOptions: [5, 10, 15, 20, -1] }"
            >
                <template v-slot:[`item.createTime`]="{ item }">
                    {{ formatUnixTimestamp(item.createTime) }}
                </template>

                <template v-slot:[`item.membersCount`]="{ item }">
                    {{ item.membersCount + "/15" }}
                </template>

                <template v-slot:[`item.controls.view`]="{ item }">
                    <v-btn x-small fab :to="'/team/' + item.tag">
                        <v-icon>mdi-eye</v-icon>
                    </v-btn>
                </template>
            </v-data-table>
        </v-card>
    </v-container>
</template>

<script>
import Queries from "../api/queries";
import FormatUtil from "../util/format-util";

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
            return FormatUtil.formatUnixTimestamp(timestamp);
        }
    },

    watch: {
        search(value) {
            console.debug(value);
        }
    },

    data: () => ({
        itemsPerPage: 10,
        search: "",
        teams: [],
        headers: [
            {
                text: "",
                value: "controls.view",
                sortable: false
            },
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
            },
            {
                text: "Members",
                value: "membersCount"
            }
        ]
    })
};
</script>
