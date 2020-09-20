<template>
    <v-container v-if="user != null">
        <simple-property-value-table :entries="userTable" />
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
    name: "UserView",

    components: {
        SimplePropertyValueTable
    },

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
                    property: "uuid",
                    value: this.user.uuid
                },
                {
                    name: "First Nick",
                    property: "firstNick",
                    value: this.user.firstNick
                },
                {
                    name: "Nick",
                    property: "nick",
                    value: this.user.nick
                },
                {
                    name: "Level",
                    property: "level",
                    value: this.user.level
                },
                {
                    name: "Experience",
                    property: "experience",
                    value: this.user.experience
                },
                {
                    name: "Owned Chests",
                    property: "chests",
                    value: this.user.chests
                },
                {
                    name: "Opened Chests",
                    property: "openedChests",
                    value: this.user.openedChests
                },
                {
                    name: "Pitcoins",
                    property: "coins",
                    value: this.user.coins
                },
                {
                    name: "Spent Pitcoins",
                    property: "spentCoins",
                    value: this.user.spentCoins
                },
                {
                    name: "Won Duels",
                    property: "wonDuels",
                    value: this.user.wonDuels
                },
                {
                    name: "Lost Duels",
                    property: "lostDuels",
                    value: this.user.lostDuels
                },
                {
                    name: "Group",
                    property: "permissionGroup",
                    value: FormatUtil.formatEnum(this.user.permissionGroup)
                },
                {
                    name: "Play Time",
                    property: "playTime",
                    value:
                        (this.user.playTime / 1000 / 60 / 60).toFixed(1) +
                        " hours (" +
                        this.user.playTime / 1000 +
                        " seconds)"
                },
                {
                    name: "Quest Progress",
                    property: "questProgress",
                    value: this.user.questProgress + "/12"
                },
                {
                    name: "First Login",
                    property: "firstLogin",
                    value: FormatUtil.formatUnixTimestamp(this.user.firstLogin)
                },
                {
                    name: "Last Login",
                    property: "lastLogin",
                    value: FormatUtil.formatUnixTimestamp(this.user.lastLogin)
                },
                {
                    name: "Last Logout",
                    property: "lastLogout",
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
