export default {
    state: () => ({
        userSearchQuery: null
    }),

    mutations: {
        setUserSearchQuery(state, value) {
            state.userSearchQuery = value;
        }
    },

    getters: {
        getUserSearchQuery(state) {
            return state.userSearchQuery;
        }
    }
};
