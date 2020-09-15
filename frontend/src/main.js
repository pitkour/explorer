import Vue from "vue";
import App from "./App.vue";
import vuetify from "./plugins/vuetify";
import apolloProvider from "./plugins/apollo";
import store from "./store";
import router from "./router";

Vue.config.productionTip = false;

const options = {
    vuetify,
    apolloProvider,
    store,
    router,
    render: h => h(App)
};

new Vue(options).$mount("#app");
