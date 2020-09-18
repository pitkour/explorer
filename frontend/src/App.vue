<template>
    <v-app>
        <menu-bar></menu-bar>

        <v-main>
            <drawer></drawer>
            <router-view />
        </v-main>
    </v-app>
</template>

<script>
import MenuBar from "./components/MenuBar";
import Drawer from "./components/Drawer";

export default {
    name: "App",
    components: {
        MenuBar,
        Drawer
    }
};
</script>
