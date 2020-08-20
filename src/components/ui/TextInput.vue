<template>
    <div class="input-wrapper">
        <input
                v-bind:type="(password && hidden) ? 'password' : 'text'"
                class="text-input"
                v-bind:placeholder="placeholder"
                v-bind:maxlength="password ? 24 : 524288"
        >
        <i v-if="password"
           class="material-icons eye no-select"
           @click="toggleVisibility()"
        >{{hidden ? "visibility" : "visibility_off"}}</i>
    </div>
</template>

<script>
    export default {
        name: "TextInput",
        props: {
            placeholder: String,
            password: Boolean
        },
        data() {
            return {
                hidden: true
            };
        },
        methods: {
            toggleVisibility() {
                this.hidden = !this.hidden;
            }
        }
    }
</script>

<style scoped>
    .input-wrapper {
        display: flex;
        align-items: center;
        width: 100%;
    }

    .text-input {
        padding: 8px;
        margin: 8px;
        width: 400px;

        border: 2px solid #dddddd;
        border-radius: 5px;
        background-color: #eeeeee;
    }

    .text-input:focus {
        border: 2px solid #008de9;
    }

    .eye {
        cursor: pointer;
        margin-left: -40px;
    }
</style>