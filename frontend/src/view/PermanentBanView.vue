<template>
    <v-container v-if="permanentBan != null">
        <simple-property-value-table
            :entries="permanentBanTable"
            :updateEntry="updateEntry"
        />
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
    name: "PermanentBanView",

    components: {
        SimplePropertyValueTable
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

    methods: {
        updateEntry(value, modifiable) {
            const uuid = this.permanentBan.uuid;
            const mutation = {
                mutation: Mutations.updatePermanentBan,

                variables: {
                    uuid,
                    [modifiable.property]:
                        modifiable.map == null ? value : modifiable.map(value)
                },

                update(store, { data: { updatePermanentBan } }) {
                    if (updatePermanentBan.affectedRows == 0) {
                        return;
                    }
                    let query = {
                        query: Queries.getPermanentBan,
                        variables: { uuid }
                    };
                    let data = store.readQuery(query);
                    data.permanentBan[modifiable.property] = value;
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
        permanentBanTable() {
            return [
                {
                    name: "UUID",
                    value: this.permanentBan.uuid,
                    view: "/user/" + this.permanentBan.uuid
                },
                {
                    name: "Nick",
                    value: this.permanentBan.nick,
                    modifiable: {
                        property: "nick"
                    }
                },
                {
                    name: "Reason",
                    value: this.permanentBan.reason,
                    modifiable: {
                        property: "reason"
                    }
                },
                {
                    name: "Performer",
                    value: this.permanentBan.performer,
                    modifiable: {
                        property: "performer"
                    }
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
