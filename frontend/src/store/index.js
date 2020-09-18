import Vue from "vue";
import Vuex from "vuex";

import drawer from "./drawer";

Vue.use(Vuex);

const store = {
    modules: {
        drawer
    }
};

export default new Vuex.Store(store);
