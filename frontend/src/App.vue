<template>
    <v-app>
        <menu-bar />

        <v-main>
            <drawer />
            <router-view class="my-8" />
        </v-main>

        <Footer />
    </v-app>
</template>

<script>
import MenuBar from "./components/MenuBar";
import Drawer from "./components/Drawer";
import Footer from "./components/Footer";

export default {
    name: "App",

    components: {
        MenuBar,
        Drawer,
        Footer
    }
};
</script>
