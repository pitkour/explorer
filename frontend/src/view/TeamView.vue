<template>
    <v-container v-if="team != null">
        <simple-property-value-table :entries="teamTable" />

        <v-card v-if="team.members.length > 0" class="mt-8 elevation-2">
            <v-card-title>Members</v-card-title>
            <v-data-table
                :headers="teamMembersHeaders"
                :items="team.members"
                :items-per-page="15"
                :footer-props="{ itemsPerPageOptions: [5, 10, 15] }"
            >
                <template v-slot:[`item.controls.view`]="{ item }">
                    <v-btn x-small fab :to="'/user/' + item.user.uuid">
                        <v-icon>mdi-eye</v-icon>
                    </v-btn>
                </template>

                <template v-slot:[`item.uuid`]="{ item }">
                    {{ item.user.uuid }}
                </template>

                <template v-slot:[`item.nick`]="{ item }">
                    {{ item.user.nick }}
                </template>

                <template v-slot:[`item.rank`]="{ item }">
                    {{ formatRank(item.rank) }}
                </template>

                <template v-slot:[`item.joinTime`]="{ item }">
                    {{ formatUnixTimestamp(item.joinTime) }}
                </template>
            </v-data-table>
        </v-card>
    </v-container>
</template>

<script>
import Queries from "../api/queries";
import FormatUtil from "../util/format-util";
import SimplePropertyValueTable from "../components/SimplePropertyValueTable";

export default {
    name: "TeamView",

    components: {
        SimplePropertyValueTable
    },

    apollo: {
        team: {
            query: Queries.getTeam,
            variables() {
                return {
                    tag: this.$route.params.tag
                };
            }
        }
    },

    methods: {
        formatUnixTimestamp(timestamp) {
            return FormatUtil.formatUnixTimestamp(timestamp);
        },

        formatRank(rank) {
            return FormatUtil.formatEnum(rank);
        }
    },

    computed: {
        teamTable() {
            return [
                {
                    name: "Tag",
                    value: this.team.tag
                },
                {
                    name: "Name",
                    value: this.team.name
                },
                {
                    name: "Creator",
                    value: this.team.creator
                },
                {
                    name: "Create Time",
                    value: FormatUtil.formatUnixTimestamp(this.team.createTime)
                },
                {
                    name: "Pitcoins Balance",
                    value: this.team.coins
                },
                {
                    name: "Members",
                    value: `${this.team.members.length}/15`
                }
            ];
        }
    },

    data: () => ({
        teamMembersHeaders: [
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
                text: "Rank",
                value: "rank"
            },
            {
                text: "Deposited Pitcoins",
                value: "coinsPaid"
            },
            {
                text: "Join Time",
                value: "joinTime"
            }
        ]
    })
};
</script>
