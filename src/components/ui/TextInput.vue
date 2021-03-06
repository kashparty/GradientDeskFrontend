<template>
    <div class="input-wrapper">
        <input
            v-bind:type="(password && hidden) ? 'password' : 'text'"
            :class="['text-input', {'disabled': disabled}]"
            :placeholder="placeholder"
            :disabled="disabled"
            v-model.lazy="value"
            @input="textChanged($event.target.value)"
            v-on:keyup.enter="$emit('enter')"
            ref="textInput"
            :maxlength="maxlength"
            :autocomplete="nocomplete ? 'new-password' : 'on'"
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
        initialValue: String,
        placeholder: String,
        password: Boolean,
        disabled: Boolean,
        autofocus: Boolean,
        maxlength: Number,
        nocomplete: Boolean
    },
    beforeMount() {
        if (this.initialValue != null) {
            this.value = this.initialValue;
        }
    },
    beforeUpdate() {
        if (this.initialValue != null) {
            this.value = this.initialValue;
        }
    },
    mounted() {
        if (this.autofocus) this.$refs.textInput.focus();
    },
    data() {
        return {
            hidden: true,
            value: "",
            prevLength: 0
        };
    },
    methods: {
        toggleVisibility() {
            this.hidden = !this.hidden;
        },
        textChanged(e) {
            this.$emit('input', e);
            if (e.length > this.prevLength) this.$refs.textInput.scrollLeft += 15;
            this.prevLength = e.length;
        }
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