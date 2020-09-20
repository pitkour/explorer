export default {
    state: () => ({
        permanentBanSearchQuery: null
    }),

    mutations: {
        setPermanentBanSearchQuery(state, value) {
            state.permanentBanSearchQuery = value;
        }
    },

    getters: {
        getPermanentBanSearchQuery(state) {
            return state.permanentBanSearchQuery;
        }
    }
};
