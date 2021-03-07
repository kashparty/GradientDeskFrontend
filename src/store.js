import Vue from "vue";
import Vuex from "vuex";

Vue.use(Vuex);

export default new Vuex.Store({
    state: {
        jwt: "",
        username: "",
        email: "",
        authorized: false,
        redirectedFrom: "/projects"
    },
    mutations: {
        initialiseStore(state) {
            if(localStorage.getItem('store')) {
				this.replaceState(
					Object.assign(state, JSON.parse(localStorage.getItem('store')))
				);
			}
        },
        authorize(state, data) {
            state.jwt = data.jwt;
            state.username = data.username;
            state.email = data.email;
            state.authorized = true;
        },
        changedUsername(state, username) {
            state.username = username;
        },
        redirect(state, route) {
            state.redirectedFrom = route;
        },
        logout(state) {
            state.jwt = "";
            state.username = "";
            state.email = "";
            state.authorized = false;
            state.redirectedFrom = "/projects";
        }
    }
});