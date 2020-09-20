<template>
    <v-container>
        <v-card class="elevation-2">
            <v-card-title>
                <v-row>
                    <v-icon class="ml-4 mr-2">
                        mdi-gavel
                    </v-icon>
                    Permanent Bans
                </v-row>

                <v-spacer />

                <search-bar :searchQuery.sync="searchQuery" />
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
import SearchBar from "../components/SearchBar";
import { mapGetters, mapMutations } from "vuex";

export default {
    name: "PermanentBanTable",

    components: {
        SearchBar
    },

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
        ...mapGetters(["getPermanentBanSearchQuery"]),
        ...mapMutations(["setPermanentBanSearchQuery"]),

        formatUnixTimestamp(timestamp) {
            return FormatUtil.formatUnixTimestamp(timestamp);
        }
    },

    computed: {
        searchQuery: {
            get() {
                return this.getPermanentBanSearchQuery();
            },

            set(value) {
                this.setPermanentBanSearchQuery(value);
            }
        }
    },

    data: () => ({
        itemsPerPage: 10,
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
