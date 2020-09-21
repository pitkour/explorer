<template>
    <v-simple-table class="elevation-2">
        <template v-slot:default>
            <thead>
                <tr>
                    <th>Property</th>
                    <th>Value</th>
                </tr>
            </thead>

            <tbody>
                <tr v-for="entry in entries" :key="entry.name">
                    <td>
                        <v-row>
                            <v-col>
                                <b>{{ entry.name }}</b>
                            </v-col>
                        </v-row>
                    </td>

                    <td
                        v-if="
                            modification != null &&
                                modification.entry.name == entry.name
                        "
                    >
                        <v-row no-gutters>
                            <v-col align-self="center" cols="4">
                                <v-text-field
                                    v-model="modification.value"
                                    dense
                                    single-line
                                    hide-details
                                ></v-text-field>
                            </v-col>

                            <v-col align-self="center">
                                <v-btn
                                    class="ml-2"
                                    small
                                    icon
                                    @click="saveModification()"
                                >
                                    <v-icon small>mdi-content-save</v-icon>
                                </v-btn>

                                <v-btn
                                    class="ml-2"
                                    small
                                    icon
                                    @click="stopModification()"
                                >
                                    <v-icon small>mdi-cancel</v-icon>
                                </v-btn>
                            </v-col>
                        </v-row>
                    </td>

                    <td v-else>
                        <v-row no-gutters>
                            <v-col align-self="center">
                                {{ entry.value }}

                                <v-btn
                                    v-if="entry.view != null"
                                    class="ml-1"
                                    small
                                    icon
                                    :to="entry.view"
                                >
                                    <v-icon small>mdi-eye</v-icon>
                                </v-btn>

                                <v-btn
                                    v-if="entry.modifiable != null"
                                    class="ml-1"
                                    small
                                    icon
                                    @click="modify(entry)"
                                >
                                    <v-icon small>mdi-pencil</v-icon>
                                </v-btn>
                            </v-col>
                        </v-row>
                    </td>
                </tr>
            </tbody>
        </template>
    </v-simple-table>
</template>

<script>
export default {
    name: "SimplePropertyValueTable",

    methods: {
        modify(entry) {
            this.modification = {
                entry,
                value: entry.value
            };
        },

        stopModification() {
            this.modification = null;
        },

        saveModification() {
            let modification = this.modification;
            this.stopModification();
            this.updateEntry(modification.value, modification.entry.modifiable);
        }
    },

    props: ["entries", "updateEntry"],

    data: () => ({
        modification: null
    })
};
</script>
