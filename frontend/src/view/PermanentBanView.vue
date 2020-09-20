<template>
    <v-container v-if="permanentBan != null">
        <permanent-ban-table :entries="permanentBanTable" />
    </v-container>
</template>

<script>
import Queries from "../api/queries";
import FormatUtil from "../util/format-util";
import PermanentBanTable from "../components/SimplePropertyValueTable";

export default {
    name: "PermanentBanView",

    components: {
        PermanentBanTable
    },

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
