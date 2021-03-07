<template>
    <div class="page-wrapper">
        <div class="page" v-if="projectInfo">
            <div></div>
            <div class="settings">
                <div class="option-row">
                    Epochs per step
                    <Slider
                        v-model="epochsPerStep"
                        :initialValue="epochsPerStep"
                        :min="1"
                        :max="10"
                        :step="1"
                    ></Slider>
                </div>
                <div class="option-row">
                    Training controls
                    <div class="button-row">
                        <Button
                            class="inline-button"
                            title="Reset"
                            @click="resetModel"
                        >
                            <i class="material-icons">replay</i>
                        </Button>
                        <Button
                            class="inline-button"
                            :title="training ? 'Pause training' : 'Train'"
                            @click="toggleTrain"
                            highlight
                        >
                            <i class="material-icons">{{
                                training ? "pause" : "fast_forward"
                            }}</i>
                        </Button>
                        <Button
                            class="inline-button"
                            title="Step"
                            @click="stepThrough"
                        >
                            <i class="material-icons">redo</i>
                        </Button>
                    </div>
                </div>
                <Plotly
                    class="chart"
                    :key="updateKey"
                    :data="chartData"
                    :layout="chartLayout"
                    :display-mode-bar="false"
                ></Plotly>
            </div>
            <div class="diagram-wrapper">
                <canvas class="connections-canvas" ref="canvas" id="canvas">
                </canvas>
                <div class="diagram-inner">
                    <div class="diagram">
                        <div
                            class="layer"
                            v-for="layer in layerInfo"
                            :key="layer.layerNumber"
                        >
                            <div
                                class="node"
                                v-for="n in layer.size"
                                :key="n"
                                @mouseenter="
                                    showConnections(
                                        `l${layer.layerNumber}_n${n}`
                                    )
                                "
                                @mouseleave="showAll"
                                :ref="`l${layer.layerNumber}_n${n}`"
                                :title="nodeName(layer.layerNumber, n)"
                            >
                                {{ n }}
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>

<script>
import firebase from "firebase/app";
import Button from "./ui/Button";
import Slider from "./ui/Slider";
import { Plotly } from "vue-plotly";

export default {
    name: "ViewProject",
    components: {
        Button,
        Slider,
        Plotly,
    },
    data() {
        return {
            projectInfo: null,
            dataprepsInfo: null,
            datasetInfo: null,

            rawData: "",
            columnData: [],
            parsedData: [],
            rows: 0,
            preparedInput: [],
            preparedOutput: [],
            layerInfo: [],
            inputNames: null,
            outputNames: null,

            epochsPerStep: 1,
            training: false,
            model: null,
            epochs: 0,

            nodeHover: null,
            chartData: [
                {
                    x: [],
                    y: [],
                    type: "line",
                },
            ],
            chartLayout: null,
            updateKey: false,

            optimiser: {
                type: "adam",
                learningRate: 0.001,
                rho: 0.9,
                beta1: 0.9,
                beta2: 0.999,
                momentumCoefficient: 0.9,
            },
        };
    },
    created() {
        this.getData();
    },
    beforeCreate() {
        if (!this.$store.state.authorized) {
            this.$store.commit(
                "redirect",
                `/projects/view/${this.$route.params.id}`
            );
            this.$router.push("/login");
            return;
        }
    },
    methods: {
        nodeName(l, n) {
            if (this.inputNames != null && this.outputNames != null) {
                if (l == 1) {
                    return this.inputNames[n - 1];
                } else if (l == this.layerInfo.length) {
                    return this.outputNames[n - 1];
                }
            }
            return "";
        },
        getData() {
            fetch(
                `https://nnvis.herokuapp.com/project/${this.$route.params.id}`,
                {
                    headers: {
                        Authorization: this.$store.state.jwt,
                    },
                }
            ).then((response) => {
                if (response.ok) {
                    response.json().then((json) => {
                        if (json.errors && json.errors.length != 0) {
                            this.$emit("toast", json.errors[0]);
                        } else {
                            this.projectInfo = json.data;
                            let optimiserData = json.data.optimiser.split(",");
                            if (optimiserData[0] == "adam") {
                                this.optimiser.type = "adam";
                                this.optimiser.learningRate = Number(
                                    optimiserData[1]
                                );
                                this.optimiser.beta1 = Number(optimiserData[2]);
                                this.optimiser.beta2 = Number(optimiserData[3]);
                            } else if (optimiserData[0] == "rmsprop") {
                                this.optimiser.type = "rmsprop";
                                this.optimiser.learningRate = Number(
                                    optimiserData[1]
                                );
                                this.optimiser.rho = Number(optimiserData[2]);
                            } else if (
                                optimiserData[0] == "nesterov" ||
                                optimiserData[0] == "momentum"
                            ) {
                                this.optimiser.type = optimiserData[0];
                                this.optimiser.learningRate = Number(
                                    optimiserData[1]
                                );
                                this.optimiser.rho = Number(optimiserData[2]);
                            } else if (optimiserData[0] == "sgd") {
                                this.optimiser.type = "sgd";
                                this.optimiser.learningRate = Number(
                                    optimiserData[1]
                                );
                            }

                            this.getDataprep();
                        }
                    });
                }
            });
        },
        getDataprep() {
            fetch(
                `https://nnvis.herokuapp.com/project/${this.$route.params.id}/dataprep`,
                {
                    headers: {
                        Authorization: this.$store.state.jwt,
                    },
                }
            ).then((response) => {
                response.json().then((json) => {
                    if (json.errors && json.errors.length != 0) {
                        this.$emit("toast", json.errors[0]);
                    } else {
                        this.dataprepsInfo = json.data;
                        if (this.dataprepsInfo.length > 0) {
                            let inputNodes = 0;
                            let outputNodes = 0;
                            for (let dataprep of this.dataprepsInfo) {
                                if (dataprep.usage == "input") {
                                    inputNodes += dataprep.nodes;
                                } else if (dataprep.usage == "output") {
                                    outputNodes += dataprep.nodes;
                                }
                            }
                            this.layerInfo.push({
                                layerNumber: 1,
                                size: inputNodes,
                                activation: "linear",
                            });
                            this.layerInfo.push({
                                layerNumber: 2,
                                size: outputNodes,
                                activation: this.projectInfo.outputActivation,
                            });
                            this.getLayerInfo();
                        }
                    }
                });
            });
        },
        getLayerInfo() {
            fetch(
                `https://nnvis.herokuapp.com/project/${this.$route.params.id}/layer`,
                {
                    headers: {
                        Authorization: this.$store.state.jwt,
                    },
                }
            ).then((response) => {
                if (response.ok) {
                    response.json().then((json) => {
                        if (json.errors && json.errors.length != 0) {
                            this.$emit("toast", json.errors[0]);
                        } else {
                            if (json.data.length > 0) {
                                this.updatingLayers = true;
                            }
                            for (let l = json.data.length - 1; l >= 0; l--) {
                                this.layerInfo.splice(1, 0, json.data[l]);
                            }
                            this.layerInfo[
                                this.layerInfo.length - 1
                            ].layerNumber += json.data.length;

                            this.createModel();
                        }
                    });
                }
            });
        },
        createModel() {
            let layers = [];
            let activationConversion = {
                linear: "Linear",
                relu: "ReLU",
                tanh: "TanH",
                leakyrelu: "LeakyReLU",
                sigmoid: "Sigmoid",
            };
            let lossConversion = {
                mse: "MeanSquaredError",
                mae: "MeanAbsoluteError",
                rmse: "RootMeanSquaredError",
                // cross: "CategoricalCrossentropy"
            };

            for (let l = 0; l < this.layerInfo.length; l++) {
                layers.push({
                    size: this.layerInfo[l].size,
                    activation:
                        activationConversion[this.layerInfo[l].activation],
                });
            }

            import("../../wasm/pkg").then((wasm) => {
                wasm.initialise();

                if (this.optimiser.type == "adam") {
                    this.model = new wasm.Adam({
                        layers: layers,
                        loss: lossConversion[this.projectInfo.loss],
                        learning_rate: this.optimiser.learningRate,
                        beta1: this.optimiser.beta1,
                        beta2: this.optimiser.beta2,
                    });
                } else if (this.optimiser.type == "rmsprop") {
                    this.model = new wasm.RMSProp({
                        layers: layers,
                        loss: lossConversion[this.projectInfo.loss],
                        learning_rate: this.optimiser.learningRate,
                        rho: this.optimiser.rho,
                    });
                } else if (this.optimiser.type == "nesterov") {
                    this.model = new wasm.Nesterov({
                        layers: layers,
                        loss: lossConversion[this.projectInfo.loss],
                        learning_rate: this.optimiser.learningRate,
                        momentum_coefficient: this.optimiser
                            .momentumCoefficient,
                    });
                } else if (this.optimiser.type == "momentum") {
                    this.model = new wasm.Momentum({
                        layers: layers,
                        loss: lossConversion[this.projectInfo.loss],
                        learning_rate: this.optimiser.learningRate,
                        momentum_coefficient: this.optimiser
                            .momentumCoefficient,
                    });
                } else if (this.optimiser.type == "sgd") {
                    this.model = new wasm.SGD({
                        layers: layers,
                        loss: lossConversion[this.projectInfo.loss],
                        learning_rate: this.optimiser.learningRate,
                    });
                }

                this.getDatasetInfo();
            });
        },
        getDatasetInfo() {
            fetch(
                `https://nnvis.herokuapp.com/dataset/${this.dataprepsInfo[0].datasetId}`,
                {
                    headers: {
                        Authorization: this.$store.state.jwt,
                    },
                }
            ).then((response) => {
                if (response.ok) {
                    response.json().then((json) => {
                        if (json.errors && json.errors.length != 0) {
                            this.$emit("toast", json.errors[0]);
                        } else {
                            this.datasetInfo = json.data;
                            if (json.data.url.startsWith("https://")) {
                                fetch(json.data.url).then((response) => {
                                    response.text().then((data) => {
                                        this.rawData = data;
                                        this.parseData();
                                    });
                                });
                            } else {
                                firebase
                                    .storage()
                                    .ref()
                                    .child(json.data.url)
                                    .getDownloadURL()
                                    .then((url) => {
                                        fetch(url).then((response) => {
                                            response.text().then((data) => {
                                                this.rawData = data;
                                                this.parseData();
                                            });
                                        });
                                    });
                            }
                        }
                    });
                }
            });
        },
        parseData() {
            fetch(
                `https://nnvis.herokuapp.com/dataset/${this.datasetInfo.datasetId}/columns`,
                {
                    headers: {
                        Authorization: this.$store.state.jwt,
                    },
                }
            ).then((response) => {
                if (response.ok) {
                    response.json().then((json) => {
                        if (json.errors && json.errors.length != 0) {
                            this.$emit("toast", json.errors[0]);
                        } else {
                            if (json.data.length > 0) {
                                this.columnData = json.data;
                                // Headers are the column names, parsedData is the data itself
                                let parsedData = [];

                                let delimiter = "";
                                if (this.datasetInfo.filetype == "csv") {
                                    delimiter = ",";
                                } else {
                                    delimiter = "\t";
                                }

                                let lines = this.rawData.split("\n");

                                // Remove blank lines at the end of the file
                                while (lines[lines.length - 1] == "")
                                    lines.pop();

                                this.rows = lines.length;
                                let cols = this.columnData.length;
                                for (let i = 0; i < cols; i++)
                                    parsedData.push([]);

                                // If reading headers from file, remove first row row and use those values as header names
                                if (this.datasetInfo.readHeaders) {
                                    lines.shift();
                                    this.rows--;
                                }

                                for (let l = 0; l < this.rows; l++) {
                                    let values = lines[l].split(delimiter);
                                    for (let v = 0; v < cols; v++) {
                                        if (this.columnData[v].include) {
                                            parsedData[v].push(
                                                values[v].trim()
                                            );
                                        }
                                    }
                                }

                                this.parsedData = parsedData;
                                this.prepareData();
                            }
                        }
                    });
                }
            });
        },
        prepareData() {
            let preparedInput = [];
            let preparedOutput = [];

            let processingInfo = {};
            let inputNames = [];
            let outputNames = [];
            for (let c = 0; c < this.columnData.length; c++) {
                if (
                    this.columnData[c].include &&
                    this.dataprepsInfo[c].usage != "none"
                ) {
                    if (this.columnData[c].type == "number") {
                        if (this.dataprepsInfo[c].usage == "input") {
                            inputNames.push(this.columnData[c].name);
                        } else if (this.dataprepsInfo[c].usage == "output") {
                            outputNames.push(this.columnData[c].name);
                        }

                        if (this.dataprepsInfo[c].normalise == "normalise") {
                            let minValue = Infinity;
                            let maxValue = 0;
                            for (let r = 0; r < this.rows; r++) {
                                minValue = Math.min(
                                    minValue,
                                    Number(this.parsedData[c][r])
                                );
                                maxValue = Math.max(
                                    maxValue,
                                    Number(this.parsedData[c][r])
                                );
                            }
                            processingInfo[c] = {
                                type: "normalise",
                                minValue: minValue,
                                maxValue: maxValue,
                            };
                        } else if (
                            this.dataprepsInfo[c].normalise == "standardise"
                        ) {
                            let sum = 0;
                            let sumSquares = 0;
                            for (let r = 0; r < this.rows; r++) {
                                sum += Number(this.parsedData[c][r]);
                                sumSquares += Math.pow(
                                    Number(this.parsedData[c][r]),
                                    2
                                );
                            }
                            let mean = sum / this.rows;
                            let std = Math.sqrt(
                                sumSquares / this.rows - Math.pow(mean, 2)
                            );
                            processingInfo[c] = {
                                type: "standardise",
                                mean: mean,
                                std: std,
                            };
                        }
                    } else if (this.columnData[c].type == "text") {
                        let conversion = {};
                        let counter = 0;
                        for (let r = 0; r < this.rows; r++) {
                            if (!(this.parsedData[c][r] in conversion)) {
                                conversion[this.parsedData[c][r]] = counter;
                                if (
                                    this.dataprepsInfo[c].encoding == "onehot"
                                ) {
                                    if (
                                        this.dataprepsInfo[c].usage == "input"
                                    ) {
                                        inputNames.push(
                                            `${this.columnData[c].name}: ${this.parsedData[c][r]}`
                                        );
                                    } else if (
                                        this.dataprepsInfo[c].usage == "output"
                                    ) {
                                        outputNames.push(
                                            `${this.columnData[c].name}: ${this.parsedData[c][r]}`
                                        );
                                    }
                                } else if (this.dataprepsInfo[c].encoding == "ordinal") {
                                    if (
                                        this.dataprepsInfo[c].usage == "input"
                                    ) {
                                        inputNames.push(
                                            `${this.columnData[c].name}`
                                        );
                                    } else if (
                                        this.dataprepsInfo[c].usage == "output"
                                    ) {
                                        outputNames.push(
                                            `${this.columnData[c].name}`
                                        );
                                    }
                                }
                                counter++;
                            }
                        }
                        processingInfo[c] = {
                            type: this.dataprepsInfo[c].encoding,
                            conversion: conversion,
                            count: counter,
                        };
                    }
                }
            }

            this.inputNames = inputNames;
            this.outputNames = outputNames;

            for (let r = 0; r < this.rows; r++) {
                let inputRowData = [];
                let outputRowData = [];
                for (let c = 0; c < this.columnData.length; c++) {
                    if (this.columnData[c].include) {
                        let multiple = false;
                        let value = this.parsedData[c][r];

                        if (this.columnData[c].type == "number") {
                            value = Number(value);
                        }

                        if (processingInfo[c]) {
                            let info = processingInfo[c];
                            if (info.type == "normalise") {
                                value =
                                    (value - info.minValue) /
                                    (info.maxValue - info.minValue);
                            } else if (info.type == "standardise") {
                                value = (value - info.mean) / info.std;
                            } else if (info.type == "ordinal") {
                                value = info.conversion[value];
                            } else if (info.type == "onehot") {
                                let arr = new Array(info.count).fill(0);
                                arr[info.conversion[value]] = 1;
                                value = arr;

                                // Adding multiple nodes due to one-hot encoding
                                multiple = true;
                            }
                        }

                        if (this.dataprepsInfo[c].usage == "input") {
                            if (!multiple) inputRowData.push(value);
                            else {
                                for (let x of value) {
                                    inputRowData.push(x);
                                }
                            }
                        } else if (this.dataprepsInfo[c].usage == "output") {
                            if (!multiple) outputRowData.push(value);
                            else {
                                for (let x of value) {
                                    outputRowData.push(x);
                                }
                            }
                        }
                    }
                }
                preparedInput.push(inputRowData);
                preparedOutput.push(outputRowData);
            }
            this.preparedInput = preparedInput;
            this.preparedOutput = preparedOutput;

            this.updateCanvas();
        },
        updateCanvas() {
            let weights = [];
            if (this.model != null) {
                weights = this.model.get_weights();
            }
            let canvas_pos = this.$refs["canvas"].getBoundingClientRect();
            let canvas = document.getElementById("canvas");
            canvas.width = canvas_pos.width;
            canvas.height = canvas_pos.height;
            let ctx = canvas.getContext("2d");
            ctx.clearRect(0, 0, canvas.width, canvas.height);

            setTimeout(() => {
                for (let l = 1; l < this.layerInfo.length; l++) {
                    for (let n1 = 1; n1 <= this.layerInfo[l - 1].size; n1++) {
                        for (let n2 = 1; n2 <= this.layerInfo[l].size; n2++) {
                            if (this.nodeHover != null) {
                                if (
                                    this.nodeHover != `l${l}_n${n1}` &&
                                    this.nodeHover != `l${l + 1}_n${n2}`
                                ) {
                                    continue;
                                }
                            }
                            let node1_pos = this.$refs[
                                `l${l}_n${n1}`
                            ][0].getBoundingClientRect();
                            let node2_pos = this.$refs[
                                `l${l + 1}_n${n2}`
                            ][0].getBoundingClientRect();

                            let rel_xpos1 =
                                node1_pos.width / 2 +
                                node1_pos.x -
                                canvas_pos.x;
                            let rel_ypos1 =
                                node1_pos.height / 2 +
                                node1_pos.y -
                                canvas_pos.y;
                            let rel_xpos2 =
                                node2_pos.width / 2 +
                                node2_pos.x -
                                canvas_pos.x;
                            let rel_ypos2 =
                                node2_pos.height / 2 +
                                node2_pos.y -
                                canvas_pos.y;

                            ctx.strokeStyle = "#333333";
                            if (weights.length > 0) {
                                let value = weights[l - 1].data[n2 - 1][n1 - 1];
                                value = Math.min(Math.max(value, -3), 3);
                                ctx.strokeStyle = `hsl(${
                                    (180 + value * 15) % 360
                                },100%,50%)`;
                                ctx.lineWidth = Math.abs(value);
                            }
                            ctx.beginPath();
                            ctx.moveTo(rel_xpos1, rel_ypos1);
                            ctx.lineTo(rel_xpos2, rel_ypos2);
                            ctx.stroke();
                        }
                    }
                }
            }, 0);
        },
        toggleTrain() {
            this.training = !this.training;

            if (this.training) {
                let inputs = {
                    rows: this.preparedInput.length,
                    cols: this.preparedInput[0].length,
                    data: this.preparedInput,
                };

                let targets = {
                    rows: this.preparedOutput.length,
                    cols: this.preparedOutput[0].length,
                    data: this.preparedOutput,
                };

                let updateGraph = () => {
                    if (this.training) {
                        this.updateKey = !this.updateKey;
                        setTimeout(updateGraph, 500);
                    }
                };

                setTimeout(updateGraph, 500);

                let train = () => {
                    this.model.forward(inputs);
                    this.model.backprop(targets);
                    this.model.update_weights(targets.rows);

                    this.chartData[0].x.push(this.epochs);
                    this.chartData[0].y.push(
                        this.model.calculate_error(targets)
                    );

                    if (this.training) {
                        this.epochs++;
                        if (this.epochs % (10 * this.epochsPerStep) == 0) {
                            this.updateCanvas();
                        }
                        setTimeout(train, 0);
                    }
                };

                setTimeout(train, 0);
            }
        },
        resetModel() {
            this.training = false;
            this.chartData = [
                {
                    x: [],
                    y: [],
                    type: "line",
                },
            ];
            this.epochs = 0;
            setTimeout(() => {
                this.createModel();
            }, 10);
        },
        stepThrough() {
            let inputs = {
                rows: this.preparedInput.length,
                cols: this.preparedInput[0].length,
                data: this.preparedInput,
            };
            let targets = {
                rows: this.preparedOutput.length,
                cols: this.preparedOutput[0].length,
                data: this.preparedOutput,
            };
            for (let i = 0; i < this.epochsPerStep; i++) {
                this.model.forward(inputs);
                this.model.backprop(targets);
                this.model.update_weights(targets.rows);

                this.chartData[0].x.push(this.epochs);
                this.chartData[0].y.push(this.model.calculate_error(targets));
                this.epochs++;
            }

            this.updateKey = !this.updateKey;
            this.updateCanvas();
        },
        showConnections(ref) {
            this.nodeHover = ref;
            this.updateCanvas();
        },
        showAll() {
            this.nodeHover = null;
            this.updateCanvas();
        },
    },
};
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
.page-wrapper {
    width: 100%;
    height: 100%;
    padding-top: 16px;
    display: flex;
    justify-content: center;
    align-items: center;
}

.page {
    width: 100%;
    height: 100%;
    display: flex;
    justify-content: space-between;
    align-items: center;
}

.settings {
    width: 30%;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    flex-shrink: 0;
}

.project-settings,
.layer-settings {
    width: 100%;
}

.button-row {
    width: 100%;
    display: flex;
    justify-content: space-evenly;
    align-items: center;
}

.option-row {
    width: 100%;
    padding-bottom: 8px;
    display: flex;
    align-items: center;
    justify-content: space-between;
}

.option-row > * {
    max-width: 60%;
}

.inline-button {
    margin-left: 8px;
    margin-right: 8px;
}

.diagram-wrapper {
    width: 60%;
    height: 100%;
    overflow: auto;
    display: flex;
    flex-direction: column;
    justify-content: center;
}

.diagram-inner {
    width: 100%;
    z-index: 1;
}

.diagram {
    display: flex;
    justify-content: space-around;
    align-items: center;
}

.layer {
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: 8px;
    border-radius: 5px;
    cursor: pointer;
}

.node {
    margin-top: 2px;
    margin-bottom: 2px;
    width: 40px;
    height: 40px;
    border: 2px solid var(--dark-blue);
    border-radius: 50px;

    display: flex;
    justify-content: center;
    align-items: center;

    background-color: #ffffff;
    z-index: 2;
}

.connections-canvas {
    height: 100%;
    width: 100%;
    position: absolute;
}

.done-button {
    display: flex;
    justify-content: center;
    padding-bottom: 8px;
}

.chart {
    width: 100%;
}
</style>
