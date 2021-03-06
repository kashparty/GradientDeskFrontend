<template>
    <div class="page"></div>
</template>

<script>
export default {
    name: "Home",
    data() {
        return {
            result: 0,
        };
    },
    // mounted() {
    //     import("../../wasm/pkg").then((wasm) => {
    //         wasm.initialise();
    //         let myNN = new wasm.NeuralNetwork({
    //             layers: [
    //                 { size: 2, activation: "Linear" },
    //                 { size: 32, activation: "TanH" },
    //                 { size: 1, activation: "Sigmoid" },
    //             ],
    //             learning_rate: 0.3,
    //             momentum_coefficient: 0.9,
    //         });

    //         let inputs = {
    //             rows: 4,
    //             cols: 2,
    //             data: [
    //                 [0, 0],
    //                 [0, 1],
    //                 [1, 0],
    //                 [1, 1],
    //             ],
    //         };

    //         let targets = {
    //             rows: 4,
    //             cols: 1,
    //             data: [[0], [1], [1], [0]],
    //         };
            
    //         let iter = 0;

    //         let train = () => {
    //             myNN.forward(inputs);
    //             myNN.backprop(targets);
    //             myNN.update_weights(targets.rows);
    //             if (iter >= 5000) {
    //                 this.result = myNN.predict(inputs).data;
    //                 console.log(myNN.calculate_error(targets));
    //             } else {
    //                 this.result = myNN.predict(inputs).data;
    //                 setTimeout(train, 0);
    //             }
    //             iter++;
    //         }
            
    //         setTimeout(train, 0);
    //     });
    // },
};
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
.page {
    padding: 16px;
    height: 100%;
    display: flex;
    justify-content: space-evenly;
    align-items: center;
}
</style>
