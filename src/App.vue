<template>
    <div id="app">
        <Header></Header>
        <transition name="fade" mode="out-in">
            <router-view v-on:toast="makeToast" />
        </transition>
        <transition name="fade" mode="out-in">
            <div class="toast-area" v-if="toasting">
                <div class="toast">{{toast}}</div>
            </div>
        </transition>
    </div>
</template>

<script>
import Header from "./components/Header";
export default {
    name: "App",
    components: {
        Header,
    },
    data() {
        return {
            toasting: false,
            toast: ""
        }
    },
    methods: {
        makeToast(message) {
            this.toasting = true;
            this.toast = message;

            setTimeout(() => {
                this.toasting = false;
                this.toast = "";
            }, 5000);
        }
    }
};
</script>

<style>
@font-face {
    font-family: Inter;
    src: url(./assets/Inter.ttf);
}

@font-face {
    font-family: "Material Icons";
    font-style: normal;
    font-weight: 400;
    src: url(./assets/MaterialIcons.ttf); /* For IE6-8 */
}
.material-icons {
    font-family: "Material Icons";
    font-weight: normal;
    font-style: normal;
    font-size: 24px; /* Preferred icon size */
    display: inline-block;
    line-height: 1;
    text-transform: none;
    letter-spacing: normal;
    word-wrap: normal;
    white-space: nowrap;
    direction: ltr;

    /* Support for all WebKit browsers. */
    -webkit-font-smoothing: antialiased;
    /* Support for Safari and Chrome. */
    text-rendering: optimizeLegibility;

    /* Support for Firefox. */
    -moz-osx-font-smoothing: grayscale;

    /* Support for IE. */
    font-feature-settings: "liga";
}

:root {
    --blue: #0093fa;
    --dark-blue: #007cd8;
    --gray: #eeeeee;
    --dark-gray: #dddddd;
    --medium-font: 20px;
    --big-font: 32px;
}

* {
    padding: 0;
    margin: 0;
    outline: none;

    font-family: Inter, Helvetica, Arial, sans-serif;
    font-size: 16px;
    font-variation-settings: "wght" 400;
    color: #333333;

    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;

    overflow-x: hidden;
}

.no-select {
    -webkit-touch-callout: none;
    -webkit-user-select: none;
    -moz-user-select: none;
    -ms-user-select: none;
    user-select: none;
}

html,
body,
#app {
    height: 100vh;
    width: 100vw;
}

#app {
    display: flex;
    flex-direction: column;

    background-color: #ffffff;
    background-image: url("data:image/svg+xml,%3Csvg width='120' height='120' viewBox='0 0 120 120' xmlns='http://www.w3.org/2000/svg'%3E%3Cpath d='M9 0h2v20H9V0zm25.134.84l1.732 1-10 17.32-1.732-1 10-17.32zm-20 20l1.732 1-10 17.32-1.732-1 10-17.32zM58.16 4.134l1 1.732-17.32 10-1-1.732 17.32-10zm-40 40l1 1.732-17.32 10-1-1.732 17.32-10zM80 9v2H60V9h20zM20 69v2H0v-2h20zm79.32-55l-1 1.732-17.32-10L82 4l17.32 10zm-80 80l-1 1.732-17.32-10L2 84l17.32 10zm96.546-75.84l-1.732 1-10-17.32 1.732-1 10 17.32zm-100 100l-1.732 1-10-17.32 1.732-1 10 17.32zM38.16 24.134l1 1.732-17.32 10-1-1.732 17.32-10zM60 29v2H40v-2h20zm19.32 5l-1 1.732-17.32-10L62 24l17.32 10zm16.546 4.16l-1.732 1-10-17.32 1.732-1 10 17.32zM111 40h-2V20h2v20zm3.134.84l1.732 1-10 17.32-1.732-1 10-17.32zM40 49v2H20v-2h20zm19.32 5l-1 1.732-17.32-10L42 44l17.32 10zm16.546 4.16l-1.732 1-10-17.32 1.732-1 10 17.32zM91 60h-2V40h2v20zm3.134.84l1.732 1-10 17.32-1.732-1 10-17.32zm24.026 3.294l1 1.732-17.32 10-1-1.732 17.32-10zM39.32 74l-1 1.732-17.32-10L22 64l17.32 10zm16.546 4.16l-1.732 1-10-17.32 1.732-1 10 17.32zM71 80h-2V60h2v20zm3.134.84l1.732 1-10 17.32-1.732-1 10-17.32zm24.026 3.294l1 1.732-17.32 10-1-1.732 17.32-10zM120 89v2h-20v-2h20zm-84.134 9.16l-1.732 1-10-17.32 1.732-1 10 17.32zM51 100h-2V80h2v20zm3.134.84l1.732 1-10 17.32-1.732-1 10-17.32zm24.026 3.294l1 1.732-17.32 10-1-1.732 17.32-10zM100 109v2H80v-2h20zm19.32 5l-1 1.732-17.32-10 1-1.732 17.32 10zM31 120h-2v-20h2v20z' fill='%23000000' fill-opacity='0.03' fill-rule='evenodd'/%3E%3C/svg%3E");
}

.fade-enter-active,
.fade-leave-active {
    transition: 0.2s opacity;
}

.fade-enter,
.fade-leave-active {
    opacity: 0;
}

.toast-area {
    width: 100%;
    display: flex;
    justify-content: flex-end;
    align-items: center;   
}

.toast {
    position: absolute;
    right: 10px;
    bottom: 10px;
    
    border: 2px solid #d33300;
    background-color: #ff3d00;
    color: #ffffff;

    padding: 8px;
    border-radius: 5px;
}
</style>
