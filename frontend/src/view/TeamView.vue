<template>
    <v-container v-if="team != null">
        <simple-property-value-table
            :entries="teamTable"
            :updateEntry="updateEntry"
        />

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
import Mutations from "../api/mutations";
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
        },

        updateEntry(value, modifiable) {
            const tag = this.team.tag;
            const mutation = {
                mutation: Mutations.updateTeam,

                variables: {
                    tag,
                    [modifiable.property]:
                        modifiable.map == null ? value : modifiable.map(value)
                },

                update(store, { data: { updateTeam } }) {
                    if (updateTeam.affectedRows == 0) {
                        return;
                    }
                    let query = {
                        query: Queries.getTeam,
                        variables: { tag }
                    };
                    let data = store.readQuery(query);
                    data.team[modifiable.property] = value;
                    store.writeQuery({
                        ...query,
                        data
                    });
                }
            };
            this.$apollo.mutate(mutation);
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
                    value: this.team.name,
                    modifiable: {
                        property: "name"
                    }
                },
                {
                    name: "Creator",
                    value: this.team.creator,
                    modifiable: {
                        property: "creator"
                    }
                },
                {
                    name: "Create Time",
                    value: FormatUtil.formatUnixTimestamp(this.team.createTime)
                },
                {
                    name: "Pitcoins Balance",
                    value: this.team.coins,
                    modifiable: {
                        property: "coins",
                        map: value => parseInt(value, 10)
                    }
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
