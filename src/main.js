import Vue from "vue";
import Vuex from "vuex";
import Chartist from "vue-chartist";

import router from "./router";
import App from "./App.vue";

Vue.config.productionTip = false;

Vue.use(Chartist);
Vue.use(Vuex);

new Vue({
    router,
    render: h => h(App),
}).$mount("#app");
