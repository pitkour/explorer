<template>
    <v-container>
        <v-simple-table v-if="user != null" class="mt-6 elevation-2">
            <template v-slot:default>
                <thead>
                    <tr>
                        <th>Property</th>
                        <th>Value</th>
                    </tr>
                </thead>
                <tbody>
                    <tr v-for="entry in userTable" :key="entry.name">
                        <td>
                            <b>{{ entry.name }}</b>
                        </td>
                        <td>{{ entry.value }}</td>
                    </tr>
                </tbody>
            </template>
        </v-simple-table>
    </v-container>
</template>

<script>
import Queries from "../api/queries";
import DateUtil from "../util/date-util";
import FormatUtil from "../util/format-util";

export default {
    name: "UserView",
    apollo: {
        user: {
            query: Queries.getUser,
            variables() {
                return {
                    uuid: this.$route.params.uuid
                };
            }
        }
    },
    computed: {
        userTable() {
            return [
                {
                    name: "UUID",
                    value: this.user.uuid
                },
                {
                    name: "First Nick",
                    value: this.user.firstNick
                },
                {
                    name: "Nick",
                    value: this.user.nick
                },
                {
                    name: "Level",
                    value: this.user.level
                },
                {
                    name: "Experience",
                    value: this.user.experience
                },
                {
                    name: "Owned Chests",
                    value: this.user.chests
                },
                {
                    name: "Opened Chests",
                    value: this.user.openedChests
                },
                {
                    name: "Pitcoins",
                    value: this.user.coins
                },
                {
                    name: "Spent Pitcoins",
                    value: this.user.spentCoins
                },
                {
                    name: "Won Duels",
                    value: this.user.wonDuels
                },
                {
                    name: "Lost Duels",
                    value: this.user.lostDuels
                },
                {
                    name: "Group",
                    value: FormatUtil.formatEnum(this.user.permissionGroup)
                },
                {
                    name: "Play Time",
                    value:
                        (this.user.playTime / 1000 / 60 / 60).toFixed(1) +
                        " hours (" +
                        this.user.playTime / 1000 +
                        " seconds)"
                },
                {
                    name: "Quest Progress",
                    value: this.user.questProgress + "/12"
                },
                {
                    name: "Team",
                    value:
                        this.user.teamMember == null
                            ? ""
                            : this.user.teamMember.tag
                },
                {
                    name: "First Login",
                    value: DateUtil.formatUnixTimestamp(this.user.firstLogin)
                },
                {
                    name: "Last Login",
                    value: DateUtil.formatUnixTimestamp(this.user.lastLogin)
                },
                {
                    name: "Last Logout",
                    value: DateUtil.formatUnixTimestamp(this.user.lastLogout)
                }
            ];
        }
    }
};
</script>
