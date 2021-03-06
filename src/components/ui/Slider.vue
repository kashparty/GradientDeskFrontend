<template>
    <label class="slider-container">
        <input
            class="slider"
            type="range"
            v-model.lazy="value"
            :min="min"
            :max="max"
            :step="step"
            @change="$emit('input', Number(value))"
        />
        <span>
            {{value}}
        </span>
    </label>
</template>

<script>
export default {
    name: "Slider",
    props: {
        initialValue: Number,
        min: Number,
        max: Number,
        step: Number
    },
    beforeMount() {
        this.value = this.initialValue;
    },
    beforeUpdate() {
        this.value = this.initialValue;
    },
    data() {
        return {
            value: 0,
        };
    },
};
</script>

<style scoped>
.slider-container {
    width: 80%;
    padding: 8px;
    display: flex;
    justify-content: space-between;
    align-items: center;
}

.slider {
    appearance: none;
    -webkit-appearance: none;
    -moz-appearance: none;
    height: 8px;
    border-radius: 50px;
    background: var(--dark-gray);
    cursor: pointer;
}

.slider::-webkit-slider-thumb {
    appearance: none;
    -webkit-appearance: none;
    background: var(--blue);
    border-color: var(--dark-blue);
    height: 16px;
    width: 16px;
    border-width: 2px;
    cursor: pointer;
}

.slider::-webkit-slider-thumb:hover {
    background: var(--dark-blue);
}

.slider::-moz-range-thumb {
    background: var(--blue);
    border-color: var(--dark-blue);
    height: 16px;
    width: 16px;
    border-radius: 50px;
    border-width: 2px;
    cursor: pointer;
}

.slider::-moz-range-thumb:hover {
    background: var(--dark-blue);
}
</style>