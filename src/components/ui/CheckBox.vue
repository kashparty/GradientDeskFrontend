<template>
    <label>
        <input
            type="checkbox"
            v-model="value"
            @change="$emit('input', $event.target.checked)"
            :checked="value"
        />
        <span>
            <i v-show="value" class="material-icons no-select">check</i>
        </span>
    </label>
</template>

<script>
export default {
    name: "CheckBox",
    props: {
        checked: Boolean,
    },
    beforeMount() {
        this.value = this.checked;
    },
    beforeUpdate() {
        this.value = this.checked;
    },
    data() {
        return {
            value: false,
        };
    },
};
</script>

<style scoped>
span {
    margin: 8px;
    width: 16px;
    height: 16px;
    background-color: #ffffff;
    border: 2px solid var(--dark-gray);
    border-radius: 5px;
    display: flex;
    justify-content: center;
    align-items: center;
    cursor: pointer;
}

span:hover {
    border-color: var(--blue);
}

input {
    display: none;
}

input:checked + span {
    background-color: var(--blue);
    border-color: var(--dark-blue);
}

i {
    font-size: 16px;
    color: #ffffff;
}
</style>