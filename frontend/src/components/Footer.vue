<template>
    <v-footer padless>
        <v-card flat tile class="text-center" width="100%">
            <v-card-text>
                <v-row no-gutters>
                    <v-col align-self="center">&copy; 2020 Piotr Dobiech</v-col>

                    <v-spacer></v-spacer>

                    <v-col>
                        <v-btn
                            v-for="buttonIcon in icons"
                            :key="buttonIcon.name"
                            icon
                            class="mx-4"
                            :href="buttonIcon.url"
                            target="_blank"
                        >
                            <v-icon>
                                {{ buttonIcon.name }}
                            </v-icon>
                        </v-btn>
                    </v-col>
                </v-row>
            </v-card-text>
        </v-card>
    </v-footer>
</template>

<script>
export default {
    name: "SiteFooter",

    data: () => ({
        icons: [
            {
                name: "mdi-github",
                url: "https://github.com/pitkour"
            },
            {
                name: "mdi-discord",
                url: "https://discord.com/invite/CcJzJCe"
            },
            {
                name: "mdi-minecraft",
                url: "https://namemc.com/server/pitkour.pl"
            }
        ]
    })
};
</script>
