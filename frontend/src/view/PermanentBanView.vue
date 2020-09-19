<template>
    <v-container>
        <v-simple-table v-if="permanentBan != null" class="elevation-2">
            <template v-slot:default>
                <thead>
                    <tr>
                        <th>Property</th>
                        <th>Value</th>
                    </tr>
                </thead>
                <tbody>
                    <tr v-for="entry in permanentBanTable" :key="entry.name">
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
    name: "PermanentBanView",

    apollo: {
        permanentBan: {
            query: Queries.getPermanentBan,
            variables() {
                return {
                    uuid: this.$route.params.uuid
                };
            }
        }
    },

    computed: {
        permanentBanTable() {
            return [
                {
                    name: "UUID",
                    value: this.permanentBan.uuid,
                    view: "/user/" + this.permanentBan.uuid
                },
                {
                    name: "Nick",
                    value: this.permanentBan.nick
                },
                {
                    name: "Reason",
                    value: this.permanentBan.reason
                },
                {
                    name: "Performer",
                    value: this.permanentBan.performer
                },
                {
                    name: "Performed",
                    value: FormatUtil.formatUnixTimestamp(
                        this.permanentBan.createTime
                    )
                }
            ];
        }
    }
};
</script>
