<template>
    <div class="page">{{result}}</div>
</template>

<script>
export default {
    name: "Home",
    data() {
        return {
            result: 0
        }
    },
    mounted() {
        import("../../wasm/pkg").then((wasm) => {
            wasm.initialise();
            let myNN = new wasm.NeuralNetwork({
                layers: [
                    { size: 2, activation: "Linear" },
                    { size: 10, activation: "TanH" },
                    { size: 1, activation: "TanH" },
                ],
                learning_rate: 0.4,
                momentum_coefficient: 0.9,
            });

            let inputs = {
                rows: 4,
                cols: 2,
                data: [[0, 0], [0, 1], [1, 0], [1, 1]]
            };

            let targets = {
                rows: 4,
                cols: 1,
                data: [[0], [1], [1], [0]]
            }

            for (let i = 0; i < 5000; i++) {
                myNN.forward(inputs);
                myNN.backprop(targets);
                myNN.update_weights(targets.rows);
            }

            this.result = myNN.predict(inputs).data;
            console.log(myNN.calculate_error(targets));

        });
    },
};
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
</style>
