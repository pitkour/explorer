import Vue from "vue";
import Vuex from "vuex";

import drawer from "./drawer";
import userTableSearchBar from "./user-table-search-bar";
import teamTableSearchBar from "./team-table-search-bar";
import permanentBanTableSearchBar from "./permanent-ban-table-search-bar";

Vue.use(Vuex);

const store = {
    modules: {
        drawer,
        userTableSearchBar,
        teamTableSearchBar,
        permanentBanTableSearchBar
    }
};

export default new Vuex.Store(store);
