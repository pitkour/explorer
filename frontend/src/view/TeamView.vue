<template>
    <v-container v-if="team != null">
        <simple-property-value-table :entries="teamTable" :tag="team.tag" />

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

    <v-container v-else>
        <v-skeleton-loader type="table" />
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
                    property: "tag",
                    value: this.team.tag
                },
                {
                    name: "Name",
                    property: "name",
                    value: this.team.name,
                    edit: value => ({
                        name: value
                    })
                },
                {
                    name: "Creator",
                    property: "creator",
                    value: this.team.creator,
                    edit: value => ({
                        creator: value
                    })
                },
                {
                    name: "Create Time",
                    property: "createTime",
                    value: FormatUtil.formatUnixTimestamp(this.team.createTime)
                },
                {
                    name: "Pitcoins Balance",
                    property: "coins",
                    value: this.team.coins,
                    edit: value => ({
                        coins: parseInt(value, 10)
                    })
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
