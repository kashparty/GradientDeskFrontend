<template>
    <div class="input-wrapper">
        <input
            v-bind:type="(password && hidden) ? 'password' : 'text'"
            :class="['text-input', {'disabled': disabled}]"
            :placeholder="placeholder"
            :disabled="disabled"
            v-model="value"
            @input="$emit('input', $event.target.value)"
            v-on:keyup.enter="$emit('enter')"
            ref="inputElement"
        />
        <i
            v-show="password"
            class="material-icons eye no-select"
            @click="toggleVisibility"
        >{{hidden ? "visibility" : "visibility_off"}}</i>
    </div>
</template>

<script>
export default {
    name: "TextInput",
    props: {
        placeholder: String,
        password: Boolean,
        disabled: Boolean,
        autofocus: Boolean
    },
    mounted() {
        if (this.autofocus) this.$refs.inputElement.focus();
    },
    data() {
        return {
            hidden: true,
            value: "",
        };
    },
    methods: {
        toggleVisibility() {
            this.hidden = !this.hidden;
        },
    },
};
</script>

<style scoped>
.input-wrapper {
    display: flex;
    align-items: center;
    width: 100%;
}

.text-input {
    padding: 8px;
    margin-top: 8px;
    margin-bottom: 8px;
    width: 95%;

    border: 2px solid var(--dark-gray);
    border-radius: 5px;
    background-color: var(--gray);
}

.text-input:focus {
    border: 2px solid var(--dark-blue);
}

.eye {
    cursor: pointer;
    margin-left: -32px;
}

.disabled {
    background-color: var(--dark-gray);
}
</style>