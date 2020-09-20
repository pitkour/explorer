<template>
    <v-text-field
        v-model="searchQueryModel"
        append-icon="mdi-magnify"
        label="Search"
        single-line
        hide-details
    ></v-text-field>
</template>

<script>
export default {
    name: "SearchBar",

    computed: {
        searchQueryModel: {
            get() {
                return this.searchQuery;
            },

            set(value) {
                let searchQuery = value ? value : null;
                this.$emit("update:searchQuery", searchQuery);
            }
        }
    },

    props: ["searchQuery"]
};
</script>
