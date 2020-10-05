import Vue from "vue";
import Chartist from "vue-chartist";
import * as firebase from "firebase/app";
import "firebase/storage";

import router from "./router.js";
import store from "./store.js";
import App from "./App.vue";

Vue.config.productionTip = false;

Vue.use(Chartist);

var firebaseConfig = {
    apiKey: "AIzaSyDNYZX_z732cv3sY5nxVLSUjchjHYp28qw",
    authDomain: "neuralvis.firebaseapp.com",
    databaseURL: "https://neuralvis.firebaseio.com",
    projectId: "neuralvis",
    storageBucket: "neuralvis.appspot.com",
    messagingSenderId: "791082575593",
    appId: "1:791082575593:web:9f54955fb02ddc6d0b7278",
};

firebase.initializeApp(firebaseConfig);

new Vue({
    router,
    store,
    render: (h) => h(App),
}).$mount("#app");
