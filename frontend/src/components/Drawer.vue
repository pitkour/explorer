<template>
    <v-navigation-drawer v-model="drawerShown" absolute temporary>
        <v-list nav dense>
            <v-list-item-group>
                <v-list-item to="/">
                    <v-list-item-icon>
                        <v-icon>mdi-home</v-icon>
                    </v-list-item-icon>
                    <v-list-item-title>Home</v-list-item-title>
                </v-list-item>

                <v-list-item to="/users">
                    <v-list-item-icon>
                        <v-icon>mdi-account-group</v-icon>
                    </v-list-item-icon>
                    <v-list-item-title>Users</v-list-item-title>
                </v-list-item>

                <v-list-item to="/teams">
                    <v-list-item-icon>
                        <v-icon>mdi-castle</v-icon>
                    </v-list-item-icon>
                    <v-list-item-title>Teams</v-list-item-title>
                </v-list-item>

                <v-list-item to="/permanent-bans">
                    <v-list-item-icon>
                        <v-icon>mdi-gavel</v-icon>
                    </v-list-item-icon>
                    <v-list-item-title>Permanent Bans</v-list-item-title>
                </v-list-item>
            </v-list-item-group>
        </v-list>
    </v-navigation-drawer>
</template>

<script>
import { mapGetters, mapMutations } from "vuex";

export default {
    name: "Drawer",
    methods: {
        ...mapGetters(["isDrawerShown"]),
        ...mapMutations(["setDrawerShown"])
    },
    computed: {
        drawerShown: {
            get() {
                return this.isDrawerShown();
            },
            set(value) {
                this.setDrawerShown(value);
            }
        }
    }
};
</script>
