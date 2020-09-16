<template>
    <v-app>
        <v-app-bar app dark>
            <div class="d-flex align-center">
                <v-img
                    alt="PitkouR"
                    class="shrink"
                    contain
                    src="https://avatars1.githubusercontent.com/u/34170911"
                    transition="scale-transition"
                    width="42"
                />
            </div>

            <div class="ml-6">
                <v-btn to="/" class="mx-2" text>
                    <span>home</span>
                </v-btn>
                <v-btn to="/users" class="mx-2" text>
                    <span>users</span>
                </v-btn>
                <v-btn to="/teams" class="mx-2" text>
                    <span>teams</span>
                </v-btn>
                <v-btn to="/permanent-bans" class="mx-2" text>
                    <span>permanent bans</span>
                </v-btn>
            </div>

            <v-spacer></v-spacer>

            <v-btn
                href="https://github.com/pitkour/explorer"
                target="_blank"
                text
            >
                <span class="mr-2">source</span>
                <v-icon>mdi-open-in-new</v-icon>
            </v-btn>
        </v-app-bar>

        <v-main>
            <router-view />
        </v-main>
    </v-app>
</template>

<script>
export default {
    name: "App"
};
</script>
