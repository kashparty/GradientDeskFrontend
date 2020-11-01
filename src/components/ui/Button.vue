<template>
    <div v-bind:class="['button', 'no-select', {'highlight': highlight, 'loading': loading}]" @click="$emit('click')">
        <slot v-if="!loading"></slot>
        <i v-else class="material-icons spinner no-select">hourglass_empty</i>
    </div>
</template>

<script>
export default {
    name: "Button",
    props: {
        highlight: Boolean,
        loading: Boolean
    },
};
</script>

<style scoped>
.button {
    display: flex;
    justify-content: center;
    align-items: center;

    padding: 8px;
    margin-top: 8px;
    margin-bottom: 8px;
    cursor: pointer;

    border-radius: 5px;
    border: 2px solid var(--gray)
}

.button:not(.loading):hover {
    background-color: var(--gray);
}

.highlight {
    border: 2px solid var(--dark-blue);
    background-color: var(--blue);
    color: #ffffff;
    font-variation-settings: "wght" 500;
}

.highlight:not(.loading):hover {
    background-color: var(--dark-blue);
}

.loading {
    cursor: default;
    background-color: transparent;
    border-color: var(--blue);
}

.spinner {
    color: var(--blue);
    animation-name: spin;
    animation-iteration-count: infinite;
    animation-duration: 1s;
    animation-timing-function: cubic-bezier(0.3, -0.7, 0.7, 1.7);
}

@keyframes spin {
    from {
        transform: rotate(0deg);
    }
    to {
        transform: rotate(360deg);
    }
}
</style>