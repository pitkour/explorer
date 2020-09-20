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
</template>

<script>
export default {
    name: "SimplePropertyValueTable",

    props: ["entries"]
};
</script>
