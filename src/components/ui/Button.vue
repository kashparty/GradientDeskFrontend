<template>
    <div v-bind:class="['button', 'no-select', {'highlight': highlight}]">
        <slot class="button-text"></slot>
    </div>
</template>

<script>
    export default {
        name: "Button",
        props: {
            highlight: Boolean
        }
    }
</script>

<style scoped>
    .button {
        display: flex;
        justify-content: center;
        align-items: center;

        padding: 8px;
        margin: 8px;
        cursor: pointer;

        border-radius: 5px;
    }

    .button:hover {
        background-color: #eeeeee;
    }

    .highlight {
        border: 2px solid #007cd8;
        background-color: #009efa;
        color: #ffffff;
        font-variation-settings: "wght" 500;
    }

    .highlight:hover {
        background-color: #007cd8;
    }
</style>