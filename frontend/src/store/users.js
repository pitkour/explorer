import Database from '../api/database'

export default {
    state: {
        all: []
    },
    mutations: {
        set(state, users) {
            state.all = users;
        }
    },
    actions: {
        get(state) {
            Database.getUsers(users => {
                state.commit('set', users);
            });
        }
    },
}
