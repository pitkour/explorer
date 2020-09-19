<template>
    <v-card>
        <v-app-bar>
            <v-app-bar-nav-icon @click="toggleDrawer()"></v-app-bar-nav-icon>

            <v-img
                alt="PitkouR"
                class="shrink ml-4"
                contain
                src="https://avatars1.githubusercontent.com/u/34170911"
                transition="scale-transition"
                width="42"
            />

            <v-toolbar-title class="ml-2">Explorer</v-toolbar-title>

            <v-spacer></v-spacer>

            <v-btn
                href="https://github.com/pitkour/explorer"
                target="_blank"
                text
            >
                <span class="mr-2">Source</span>
                <v-icon>mdi-open-in-new</v-icon>
            </v-btn>
        </v-app-bar>
    </v-card>
</template>

<script>
import { mapMutations } from "vuex";

export default {
    name: "MenuBar",
    methods: {
        ...mapMutations(["toggleDrawer"])
    }
};
</script>
