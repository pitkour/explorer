export default {
    state: () => ({
        teamSearchQuery: null
    }),

    mutations: {
        setTeamSearchQuery(state, value) {
            state.teamSearchQuery = value;
        }
    },

    getters: {
        getTeamSearchQuery(state) {
            return state.teamSearchQuery;
        }
    }
};
