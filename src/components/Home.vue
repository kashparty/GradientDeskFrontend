<template>
    <div class="page"></div>
</template>

<script>
export default {
    name: "Home",
    mounted() {
        import("../../wasm/pkg").then((nn) => {
            nn.initialise();
            console.log(nn);
        })
    },
};
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
</style>
