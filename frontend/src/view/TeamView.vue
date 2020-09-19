<template>
    <v-container v-if="team != null">
        <v-simple-table class="mt-6 elevation-2">
            <template v-slot:default>
                <thead>
                    <tr>
                        <th>Property</th>
                        <th>Value</th>
                    </tr>
                </thead>

                <tbody>
                    <tr v-for="entry in teamTable" :key="entry.name">
                        <td>
                            <b>{{ entry.name }}</b>
                        </td>
                        <td>{{ entry.value }}</td>
                    </tr>
                </tbody>
            </template>
        </v-simple-table>

        <v-card v-if="team.members.length > 0" class="mt-8 elevation-2">
            <v-card-title>Members</v-card-title>
            <v-simple-table>
                <template v-slot:default>
                    <thead>
                        <tr>
                            <th>UUID</th>
                            <th>Nick</th>
                            <th>Rank</th>
                            <th>Deposited Pitcoins</th>
                            <th>Join Time</th>
                        </tr>
                    </thead>

                    <tbody>
                        <tr v-for="member in team.members" :key="member.uuid">
                            <td>{{ member.user.uuid }}</td>
                            <td>{{ member.user.nick }}</td>
                            <td>{{ formatRank(member.rank) }}</td>
                            <td>{{ member.coinsPaid }}</td>
                            <td>{{ formatUnixTimestamp(member.joinTime) }}</td>
                        </tr>
                    </tbody>
                </template>
            </v-simple-table>
        </v-card>
    </v-container>
</template>

<script>
import Queries from "../api/queries";
import FormatUtil from "../util/format-util";

export default {
    name: "TeamView",

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
    }
};
</script>
