<template>
    <div class="header-bar">
        <router-link to="/" class="header-title">Interact with Neural Networks</router-link>
        <div class="header-links">
            <router-link
                    to="/login"
                    v-bind:class="['link', {'current': this.$route.path === '/login'}]"
            >Log in</router-link>
            <router-link
                    to="/datasets"
                    v-bind:class="['link', {'current': this.$route.path === '/datasets'}]"
            >Datasets</router-link>
        </div>
    </div>
</template>

<script>
    export default {
        name: "Header"
    }
</script>

<style scoped>
    .header-bar {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 16px;
    }

    .header-title {
        font-size: 20px;
        font-variation-settings: "wght" 500;
        cursor: pointer;
        text-decoration: none;
    }

    .header-links {
        display: flex;
    }

    .link {
        margin-right: 16px;
        cursor: pointer;
        text-decoration: none;
        padding: 8px;
        border-radius: 5px;
    }

    .link:hover {
        background-color: #eeeeee;
    }

    .current {
        text-decoration: underline;
    }
</style>