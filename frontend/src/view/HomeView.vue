<template>
    <v-container fill-height fluid>
        <v-row class="text-center">
            <v-col cols="12">
                <h1 class="mb-2">Explorer</h1>
                <p>Explore the data stored in PitkouR's database!</p>
                <iframe
                    class="mt-4"
                    style="border: none"
                    src="https://namemc.com/server/pitkour.pl/embed"
                    width="500"
                ></iframe>
            </v-col>
        </v-row>
    </v-container>
</template>

<script>
export default {
    name: "Home"
};
</script>
