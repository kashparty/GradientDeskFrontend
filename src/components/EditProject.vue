<template>
    <div class="page-wrapper">
        <div class="page" v-if="projectInfo">
            <div></div>
            <div class="settings">
                <div class="project-settings" v-if="editingLayer == null">
                    <div class="option-row">
                        Project name
                        <TextInput
                            v-model="projectInfo.name"
                            :placeholder="projectInfo.name"
                            :initialValue="projectInfo.name"
                            :maxlength="32"
                        ></TextInput>
                    </div>
                    <div class="option-row">
                        Project description
                        <TextInput
                            v-model="projectInfo.description"
                            :placeholder="projectInfo.description"
                            :initialValue="projectInfo.description"
                            :maxlength="128"
                        ></TextInput>
                    </div>
                    <div class="option-row" v-if="!alreadyPrepped">
                        No dataset prepared
                        <Button @click="selectDataset" highlight
                            >Prepare dataset</Button
                        >
                    </div>
                    <div class="option-row" v-else-if="datasetInfo">
                        "{{ datasetInfo.name }}" dataset prepared
                        <div class="button-row">
                            <Button
                                class="inline-button"
                                @click="removeDataset"
                            >
                                <i class="material-icons">delete</i>
                            </Button>
                            <Button
                                class="inline-button"
                                @click="editDatapreps"
                            >
                                <i class="material-icons">edit</i>
                            </Button>
                        </div>
                    </div>
                    <div class="option-row" v-if="layerInfo.length > 0">
                        Number of layers
                        <div class="button-row">
                            <Button
                                class="inline-button"
                                @click="removeLayer"
                                :disabled="layerInfo.length < 3"
                            >
                                <i class="material-icons">remove</i>
                            </Button>
                            {{ layerInfo.length }}
                            <Button
                                class="inline-button"
                                @click="addLayer"
                                :disabled="layerInfo.length >= 6"
                            >
                                <i class="material-icons">add</i>
                            </Button>
                        </div>
                    </div>
                    <div class="option-row">
                        Project setting
                        <select v-model="projectSetting">
                            <option value="batchsize">Batch size</option>
                            <option value="optimiser">Optimiser</option>
                            <option value="loss">Loss</option>
                        </select>
                    </div>
                    <div
                        class="option-row"
                        v-if="projectSetting == 'batchsize'"
                    >
                        Batch size
                        <Slider
                            v-model="projectInfo.batchSize"
                            :initialValue="projectInfo.batchSize"
                            :min="0"
                            :max="128"
                            :step="4"
                        ></Slider>
                    </div>
                    <div class="option-row" v-if="projectSetting == 'loss'">
                        Loss
                        <select v-model="projectInfo.loss">
                            <option value="mse">Mean Squared Error</option>
                            <option value="mae">Mean Absolute Error</option>
                            <option value="rmse">
                                Root Mean Squared Error
                            </option>
                            <!-- <option value="cross">Categorical cross-entropy</option> -->
                        </select>
                    </div>
                    <div
                        class="option-row"
                        v-if="projectSetting == 'optimiser'"
                    >
                        Optimiser
                        <select v-model="optimiser.type">
                            <option value="adam">Adam</option>
                            <option value="rmsprop">RMSProp</option>
                            <option value="nesterov">
                                Nesterov Accelerated Gradient
                            </option>
                            <option value="momentum">Momentum</option>
                            <option value="sgd">
                                Stochastic Gradient Descent
                            </option>
                        </select>
                    </div>
                    <div
                        class="option-row"
                        v-if="projectSetting == 'optimiser'"
                    >
                        Learning rate
                        <TextInput
                            v-model.number="optimiser.learningRate"
                            :placeholder="optimiser.learningRate.toString()"
                            :initialValue="optimiser.learningRate.toString()"
                            :maxlength="8"
                        ></TextInput>
                    </div>
                    <div
                        class="option-row"
                        v-if="
                            projectSetting == 'optimiser' &&
                            optimiser.type == 'adam'
                        "
                    >
                        Beta 1
                        <TextInput
                            v-model.number="optimiser.beta1"
                            :placeholder="optimiser.beta1.toString()"
                            :initialValue="optimiser.beta1.toString()"
                            :maxlength="8"
                        ></TextInput>
                    </div>
                    <div
                        class="option-row"
                        v-if="
                            projectSetting == 'optimiser' &&
                            optimiser.type == 'adam'
                        "
                    >
                        Beta 2
                        <TextInput
                            v-model.number="optimiser.beta2"
                            :placeholder="optimiser.beta2.toString()"
                            :initialValue="optimiser.beta2.toString()"
                            :maxlength="8"
                        ></TextInput>
                    </div>
                    <div
                        class="option-row"
                        v-if="
                            projectSetting == 'optimiser' &&
                            optimiser.type == 'rmsprop'
                        "
                    >
                        Rho
                        <TextInput
                            v-model.number="optimiser.rho"
                            :placeholder="optimiser.rho.toString()"
                            :initialValue="optimiser.rho.toString()"
                            :maxlength="8"
                        ></TextInput>
                    </div>
                    <div
                        class="option-row"
                        v-if="
                            projectSetting == 'optimiser' &&
                            (optimiser.type == 'nesterov' ||
                                optimiser.type == 'momentum')
                        "
                    >
                        Momentum coefficient
                        <TextInput
                            v-model.number="optimiser.momentumCoefficient"
                            :placeholder="optimiser.momentumCoefficient.toString()"
                            :initialValue="optimiser.momentumCoefficient.toString()"
                            :maxlength="8"
                        ></TextInput>
                    </div>
                    <div class="button-row">
                        <Button @click="deleteProject">Delete project</Button>
                        <Button @click="doneEditing" highlight
                            >Done editing</Button
                        >
                    </div>
                </div>
                <div class="layer-settings" v-else>
                    <div class="option-row" v-if="editingLayer < layerInfo.length">
                        Number of nodes
                        <div class="button-row">
                            <Button
                                class="inline-button"
                                @click="layerInfo[editingLayer - 1].size--"
                                :disabled="layerInfo[editingLayer - 1].size < 2"
                            >
                                <i class="material-icons">remove</i>
                            </Button>
                            {{ layerInfo[editingLayer - 1].size }}
                            <Button
                                class="inline-button"
                                @click="layerInfo[editingLayer - 1].size++"
                                :disabled="
                                    layerInfo[editingLayer - 1].size >= 10
                                "
                            >
                                <i class="material-icons">add</i>
                            </Button>
                        </div>
                    </div>
                    <div class="option-row">
                        Activation function
                        <select
                            v-model="layerInfo[editingLayer - 1].activation"
                        >
                            <option value="linear">Linear</option>
                            <option value="sigmoid">Sigmoid</option>
                            <option value="tanh">Tanh</option>
                            <option value="relu">ReLU</option>
                            <option value="leakyrelu">Leaky ReLU</option>
                        </select>
                    </div>
                    <div class="done-button">
                        <Button @click="editLayerDone" highlight>Done</Button>
                    </div>
                </div>
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
                            @click="editLayer(layer.layerNumber)"
                        >
                            <div
                                class="node"
                                v-for="n in layer.size"
                                :key="n"
                                :ref="`l${layer.layerNumber}_n${n}`"
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
import Button from "./ui/Button";
import TextInput from "./ui/TextInput";
import Slider from "./ui/Slider";

export default {
    name: "EditProject",
    components: {
        Button,
        TextInput,
        Slider,
    },
    data() {
        return {
            projectInfo: null,
            dataprepsInfo: null,
            datasetInfo: null,
            alreadyPrepped: false,

            editingLayer: null,
            layerInfo: [],
            updatingLayers: false,

            projectSetting: "batchsize",
            optimiser: {
                type: "adam",
                learningRate: 0.001,
                rho: 0.9,
                beta1: 0.9,
                beta2: 0.999,
                momentumCoefficient: 0.9
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
                `/projects/edit/${this.$route.params.id}`
            );
            this.$router.push("/login");
            return;
        }
    },
    watch: {
        layerInfo: {
            handler: function () {
                this.updateCanvas();
            },
            deep: true,
        },
    },
    methods: {
        updateSetting() {
            alert();
        },
        getData() {
            fetch(`https://nnvis.herokuapp.com/project/${this.$route.params.id}`, {
                headers: {
                    Authorization: this.$store.state.jwt,
                },
            }).then((response) => {
                if (response.ok) {
                    response.json().then((json) => {
                        if (json.errors && json.errors.length != 0) {
                            this.$emit("toast", json.errors[0]);
                        } else {
                            this.projectInfo = json.data;
                            let optimiserData = json.data.optimiser.split(",");
                            if (optimiserData[0] == "adam") {
                                this.optimiser.type = "adam";
                                this.optimiser.learningRate = Number(optimiserData[1]);
                                this.optimiser.beta1 = Number(optimiserData[2]);
                                this.optimiser.beta2 = Number(optimiserData[3]);
                            } else if (optimiserData[0] == "rmsprop") {
                                this.optimiser.type = "rmsprop";
                                this.optimiser.learningRate = Number(optimiserData[1]);
                                this.optimiser.rho = Number(optimiserData[2]);
                            } else if (optimiserData[0] == "nesterov" || optimiserData[0] == "momentum") {
                                this.optimiser.type = optimiserData[0];
                                this.optimiser.learningRate = Number(optimiserData[1]);
                                this.optimiser.rho = Number(optimiserData[2]);
                            } else if (optimiserData[0] == "sgd") {
                                this.optimiser.type = "sgd";
                                this.optimiser.learningRate = Number(optimiserData[1]);
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
                            this.alreadyPrepped = true;
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
                            this.getDatasetInfo();
                            this.updateCanvas();
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
                        }
                    });
                }
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
                        }
                    });
                }
            });
        },
        selectDataset() {
            this.$router.push(`/datasets/select/${this.$route.params.id}`);
        },
        removeDataset() {
            fetch(
                `https://nnvis.herokuapp.com/project/${this.$route.params.id}/dataprep`,
                {
                    method: "DELETE",
                    headers: {
                        Authorization: this.$store.state.jwt,
                    },
                }
            ).then((response) => {
                response.json().then((json) => {
                    if (json.errors && json.errors.length != 0) {
                        this.$emit("toast", json.errors[0]);
                    } else {
                        this.datasetInfo = null;
                        this.dataprepsInfo = null;
                        this.alreadyPrepped = false;
                        this.layerInfo = [];
                        this.updateCanvas();
                    }
                });
            });
        },
        editDatapreps() {
            this.$router.push(
                `/datasets/prepare/${this.projectInfo.projectId}/${this.datasetInfo.datasetId}`
            );
        },
        deleteProject() {
            fetch(`https://nnvis.herokuapp.com/project/${this.$route.params.id}`, {
                method: "DELETE",
                headers: {
                    Authorization: this.$store.state.jwt,
                },
            }).then((response) => {
                response.json().then((json) => {
                    if (json.errors && json.errors.length != 0) {
                        this.$emit("toast", json.errors[0]);
                    } else {
                        this.$router.push("/projects");
                    }
                });
            });
        },
        doneEditing() {
            let optimiserString = `${this.optimiser.type},${this.optimiser.learningRate},`;
            if (this.optimiser.type == "adam") {
                optimiserString += `${this.optimiser.beta1},${this.optimiser.beta2}`;
            } else if (this.optimiser.type == "rmsprop") {
                optimiserString += `${this.optimiser.rho}`;
            } else if (this.optimiser.type == "nesterov" || this.optimiser.type == "momentum") {
                optimiserString += `${this.optimiser.momentumCoefficient}`;
            }
            this.projectInfo.optimiser = optimiserString;
            this.projectInfo.outputActivation = this.layerInfo[this.layerInfo.length - 1].activation;

            fetch("https://nnvis.herokuapp.com/project", {
                method: "PUT",
                headers: {
                    "Content-Type": "application/json",
                    Authorization: this.$store.state.jwt,
                },
                body: JSON.stringify(this.projectInfo),
            }).then((response) => {
                if (response.ok) {
                    response.json().then((json) => {
                        if (json.errors && json.errors.length != 0) {
                            console.log(json.errors);
                            this.$emit("toast", json.errors.join(", "));
                        } else {
                            fetch("https://nnvis.herokuapp.com/project/layer", {
                                method: this.updatingLayers ? "PUT" : "POST",
                                headers: {
                                    "Content-Type": "application/json",
                                    Authorization: this.$store.state.jwt,
                                },
                                body: JSON.stringify({
                                    projectId: this.$route.params.id,
                                    data: this.layerInfo.slice(
                                        1,
                                        this.layerInfo.length - 1
                                    ),
                                }),
                            }).then((response) => {
                                if (response.ok) {
                                    response.json().then((json) => {
                                        if (
                                            json.errors &&
                                            json.errors.length != 0
                                        ) {
                                            console.log(json.errors);
                                            this.$emit(
                                                "toast",
                                                json.errors.join(", ")
                                            );
                                        } else {
                                            this.$router.push("/projects");
                                        }
                                    });
                                }
                            });
                        }
                    });
                }
            });
        },
        editLayer(layerNumber) {
            if (layerNumber > 1) {
                this.editingLayer = layerNumber;
            }
        },
        editLayerDone() {
            this.editingLayer = null;
        },
        updateCanvas() {
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

                            // let value = Math.random();
                            // ctx.strokeStyle = "rgba(51, 51, 51, " + value + ")";
                            // ctx.lineWidth = value * 2;
                            ctx.strokeStyle = "#333333";
                            ctx.beginPath();
                            ctx.moveTo(rel_xpos1, rel_ypos1);
                            ctx.lineTo(rel_xpos2, rel_ypos2);
                            ctx.stroke();
                        }
                    }
                }
            }, 0);
        },
        addLayer() {
            this.layerInfo.splice(this.layerInfo.length - 1, 0, {
                layerNumber: this.layerInfo.length,
                size: 4,
                activation: "tanh",
            });
            this.layerInfo[this.layerInfo.length - 1].layerNumber++;
            this.updateCanvas();
        },
        removeLayer() {
            this.layerInfo.splice(this.layerInfo.length - 2, 1);
            this.layerInfo[this.layerInfo.length - 1].layerNumber--;
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

.layer:hover .node {
    background-color: var(--blue);
    color: #ffffff;
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
</style>
