<template>
    <v-container>
        <v-card class="elevation-2">
            <v-card-title>
                <v-row>
                    <v-icon class="ml-4 mr-2">mdi-account-group</v-icon>
                    Users
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
                :items="users"
                :items-per-page="itemsPerPage"
                :footer-props="{ itemsPerPageOptions: [5, 10, 15, 20, -1] }"
            >
                <template v-slot:[`item.firstLogin`]="{ item }">
                    {{ formatUnixTimestamp(item.firstLogin) }}
                </template>

                <template v-slot:[`item.controls.view`]="{ item }">
                    <v-btn x-small fab :to="'/user/' + item.uuid">
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
        users: [],
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
