<template>
    <div class="header-bar">
        <div class="header-logo">
            <router-link to="/"
                ><img class="logo" src="../assets/logo.png"
            /></router-link>
        </div>
        <div class="header-title no-select">GradientDesk {{this.$route.meta.title ? ` - ${this.$route.meta.title}` : ""}}</div>
        <div class="header-links">
            <router-link
                to="/projects"
                v-bind:class="[
                    'link',
                    { current: this.$route.path === '/projects' },
                ]"
                >Projects</router-link
            >
            <router-link
                to="/datasets"
                v-bind:class="[
                    'link',
                    { current: this.$route.path === '/datasets' },
                ]"
                >Datasets</router-link
            >
            <router-link
                to="/account"
                v-bind:class="[
                    'link',
                    { current: this.$route.path === '/account' },
                ]"
                >Account</router-link
            >
        </div>
    </div>
</template>

<script>
export default {
    name: "Header",
};
</script>

<style scoped>
.header-bar {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 16px;
}

.header-title {
    font-size: var(--medium-font);
    font-variation-settings: "wght" 600;
    text-decoration: none;
}

.header-links {
    display: flex;
    justify-content: flex-end;
    align-items: center;
    width: 30%;
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
    text-decoration-thickness: 2px;
    text-decoration-color: var(--blue);
}

.header-logo {
    width: 30%;
}

.logo {
    width: 32px;
    padding: 8px;
    filter: grayscale(1) brightness(0.5);
}

.logo:hover {
    filter: none;
}
</style>