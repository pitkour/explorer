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
                        <td>
                            {{ entry.value }}
                            <v-btn
                                v-if="entry.view != null"
                                class="ml-1"
                                small
                                icon
                                :to="entry.view"
                            >
                                <v-icon>mdi-eye</v-icon>
                            </v-btn>
                        </td>
                    </tr>
                </tbody>
            </template>
        </v-simple-table>
    </v-container>
</template>

<script>
import Queries from "../api/queries";
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
                    name: "First Login",
                    value: FormatUtil.formatUnixTimestamp(this.user.firstLogin)
                },
                {
                    name: "Last Login",
                    value: FormatUtil.formatUnixTimestamp(this.user.lastLogin)
                },
                {
                    name: "Last Logout",
                    value: FormatUtil.formatUnixTimestamp(this.user.lastLogout)
                },
                {
                    name: "Team",
                    value:
                        this.user.teamMember == null
                            ? "N/A"
                            : this.user.teamMember.tag,

                    view:
                        this.user.teamMember == null
                            ? null
                            : "/team/" + this.user.teamMember.tag
                },
                {
                    name: "Permanent Banned",
                    value: FormatUtil.formatBoolean(
                        this.user.permanentBan != null
                    ),
                    view:
                        this.user.permanentBan == null
                            ? null
                            : "/permanent-ban/" + this.user.permanentBan.uuid
                }
            ];
        }
    }
};
</script>
