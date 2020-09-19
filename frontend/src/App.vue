<template>
    <v-app>
        <menu-bar />

        <v-main>
            <drawer />
            <router-view class="my-8" />
        </v-main>

        <site-footer />
    </v-app>
</template>

<script>
import MenuBar from "./components/MenuBar";
import Drawer from "./components/Drawer";
import SiteFooter from "./components/SiteFooter";

export default {
    name: "App",

    components: {
        MenuBar,
        Drawer,
        SiteFooter
    }
};
</script>
