import Vue from "vue";
import * as firebase from "firebase/app";
import "firebase/storage";

import router from "./router.js";
import store from "./store.js";
import App from "./App.vue";

store.subscribe((mutation, state) => {
	// Store the state object as a JSON string
	localStorage.setItem('store', JSON.stringify(state));
});

Vue.config.productionTip = false;

var firebaseConfig = {
    apiKey: "its_gone_now",
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
