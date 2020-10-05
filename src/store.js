import Vue from "vue";
import Vuex from "vuex";

Vue.use(Vuex);

export default new Vuex.Store({
    state: {
        jwt: "",
        authorized: false,
        redirectedFrom: "/projects"
    },
    mutations: {
        authorize(state, jwt) {
            state.jwt = jwt;
            state.authorized = true;
        },
        redirect(state, route) {
            state.redirectedFrom = route;
        }
    }
});
