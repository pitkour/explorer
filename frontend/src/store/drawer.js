export default {
    state: () => ({
        drawerShown: false
    }),

    mutations: {
        setDrawerShown(state, value) {
            state.drawerShown = value;
        },

        toggleDrawer(state) {
            state.drawerShown = !state.drawerShown;
        }
    },

    getters: {
        isDrawerShown(state) {
            return state.drawerShown;
        }
    }
};
