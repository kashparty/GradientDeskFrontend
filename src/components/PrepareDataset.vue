<template>
    <div class="page" v-if="datasetInfo && projectInfo && dataprepInfo">
        <div class="workspace">
            <div class="settings">
                <!--Settings for whole table-->
                <div class="column-settings" v-if="editColNum === null">
                    <div class="option-row">
                        Randomly shuffle rows
                        <CheckBox
                            v-model="projectInfo.shuffle"
                            :checked="projectInfo.shuffle"
                        ></CheckBox>
                    </div>
                    <div class="option-row">
                        Exclude corrupt rows
                        <CheckBox
                            v-model="projectInfo.excludeCorrupt"
                            :checked="projectInfo.excludeCorrupt"
                        ></CheckBox>
                    </div>
                    <div class="option-row">
                        Test split ratio
                        <Slider
                            v-model="projectInfo.testSplit"
                            :initialValue="projectInfo.testSplit"
                            :min="0"
                            :max="1"
                            :step="0.05"
                        ></Slider>
                    </div>
                    <div class="button-row">
                        <div class="done-button">
                            <Button @click="cancelChanges">Cancel</Button>
                        </div>
                        <div class="done-button">
                            <Button @click="saveChanges" highlight
                                >Save changes</Button
                            >
                        </div>
                    </div>
                </div>

                <!--Column-specific settings, only shown when editing a column-->
                <div v-else class="column-settings">
                    <div class="option-row">
                        Input/Output
                        <select v-model="dataprepInfo[editColNum].usage">
                            <option value="unused">Unused</option>
                            <option value="input">Input</option>
                            <option value="output">Output</option>
                        </select>
                    </div>
                    <div v-if="dataprepInfo[editColNum].usage != 'unused'">
                        <div
                            class="option-row"
                            v-if="columnData[editColNum].type == 'number'"
                        >
                            Normalisation/Standardisation
                            <select
                                v-model="dataprepInfo[editColNum].normalise"
                            >
                                <option value="none">None</option>
                                <option value="normalise">Normalise</option>
                                <option value="standardise">Standardise</option>
                            </select>
                        </div>
                        <div class="option-row" v-else>
                            Categorical encoding
                            <select v-model="dataprepInfo[editColNum].encoding">
                                <option value="ordinal">Ordinal</option>
                                <option value="onehot">One-hot</option>
                            </select>
                        </div>
                    </div>
                    <div class="done-button">
                        <Button @click="editColDone" highlight>Done</Button>
                    </div>
                </div>
            </div>
            <div class="table-wrapper">
                <table>
                    <tr>
                        <!--Column headers, including edit icons-->
                        <th
                            v-for="(col, c) in columnData.filter(
                                (c) => c.include
                            )"
                            :key="c"
                            :class="{ used: dataprepInfo[col.index].usage != 'unused' }"
                        >
                            <div class="header-option">
                                <i class="material-icons no-select">
                                    {{
                                        col.type == "number"
                                            ? "scatter_plot"
                                            : "notes"
                                    }}
                                </i>
                                <div class="header-name">
                                    {{ col.name }}
                                </div>
                                <i
                                    class="material-icons no-select"
                                    @click="editCol(col.index)"
                                    >edit</i
                                >
                            </div>
                        </th>
                    </tr>
                    <tr v-for="r in Math.min(rows, 50)" :key="r">
                        <!--Actual column data. Note col[r - 1] is used since Vue for loops start at 1.-->
                        <td
                            v-for="(col, c) in columnData.filter(
                                (c) => c.include
                            )"
                            :key="c"
                        >
                            {{ parsedData[col.index][r - 1] }}
                        </td>
                    </tr>
                </table>
            </div>
        </div>
    </div>
</template>

<script>
import firebase from "firebase/app";
import Button from "./ui/Button";
import CheckBox from "./ui/CheckBox";
import Slider from "./ui/Slider";

export default {
    name: "PrepareDataset",
    components: {
        Button,
        CheckBox,
        Slider,
    },
    data() {
        return {
            datasetInfo: null,
            rawData: "",
            dataprepInfo: null,
            projectInfo: null,

            columnData: [],
            parsedData: [],
            rows: 0,
            updatingPreps: false,

            editColNum: null,
        };
    },
    created() {
        this.getData();
    },
    beforeCreate() {
        if (!this.$store.state.authorized) {
            this.$store.commit(
                "redirect",
                `/datasets/prepare/${this.$route.params.projectId}/${this.$route.params.datasetId}`
            );
            this.$router.push("/login");
            return;
        }
    },
    methods: {
        getData() {
            fetch(
                `https://nnvis.herokuapp.com/dataset/${this.$route.params.datasetId}`,
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

            fetch(
                `https://nnvis.herokuapp.com/project/${this.$route.params.projectId}`,
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
                        }
                    });
                }
            });
        },
        parseData() {
            fetch(
                `https://nnvis.herokuapp.com/dataset/${this.$route.params.datasetId}/columns`,
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
                                this.getDataPreps();
                            }
                        }
                    });
                }
            });
        },
        getDataPreps() {
            fetch(
                `https://nnvis.herokuapp.com/project/${this.$route.params.projectId}/dataprep`,
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
                        if (json.data.length > 0) {
                            this.dataprepInfo = json.data;
                            this.updatingPreps = true;
                        } else {
                            this.dataprepInfo = [];
                            for (let c of this.columnData) {
                                this.dataprepInfo.push({
                                    projectId: this.$route.params.projectId,
                                    datasetId: this.$route.params.datasetId,
                                    index: c.index,
                                    usage: "unused",
                                    normalise: "none",
                                    encoding:
                                        c.type == "number" ? "none" : "onehot",
                                    nodes: 1,
                                });
                            }
                        }
                    }
                });
            });
        },
        editCol(colNumber) {
            this.editColNum = colNumber;
        },
        editColDone() {
            this.editColNum = null;
        },
        saveChanges() {
            for (let i = 0; i < this.dataprepInfo.length; i++) {
                if (this.dataprepInfo[i].encoding == "onehot" && this.columnData[i].include) {
                    this.dataprepInfo[i].nodes = new Set(this.parsedData[i]).size;
                }
            }

            console.log(this.dataprepInfo);

            fetch("https://nnvis.herokuapp.com/project", {
                method: "PUT",
                headers: {
                    "Content-Type": "application/json",
                    Authorization: this.$store.state.jwt,
                },
                body: JSON.stringify(this.projectInfo),
            });
            fetch(`https://nnvis.herokuapp.com/project/dataprep`, {
                method: this.updatingPreps ? "PUT" : "POST",
                headers: {
                    "Content-Type": "application/json",
                    Authorization: this.$store.state.jwt,
                },
                body: JSON.stringify({
                    projectId: this.$route.params.projectId,
                    data: this.dataprepInfo,
                }),
            });
            this.$router.push(`/projects/edit/${this.$route.params.projectId}`);
        },
        cancelChanges() {
            this.$router.back();
        },
    },
};
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
.page {
    padding-top: 16px;
    height: 100%;
    display: flex;
    flex-direction: column;
}

.page-title {
    text-align: center;
    font-size: var(--big-font);
    margin-bottom: 24px;
    font-variation-settings: "wght" 600;
}

.workspace {
    display: flex;
    flex-direction: column;
}

.settings {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    flex-shrink: 0;
}

.table-wrapper {
    overflow: auto;
    height: 100%;
    width: 100%;
    display: flex;
}

table,
tr,
th,
td {
    flex-shrink: 0;
    border: 2px solid var(--dark-gray);
    border-collapse: collapse;
    margin-left: auto;
    margin-right: auto;
}

td {
    padding: 8px;
    min-width: 200px;
    text-align: center;
    background-color: #ffffff;
}

th {
    position: sticky;
    top: 2px;
    background-color: #888888;
    box-shadow: 0 -3px 0 1px #888888;
    z-index: 10;
}

th.used {
    background-color: var(--blue);
    box-shadow: 0 -3px 0 1px var(--blue);
}

.option-row {
    padding-bottom: 8px;
    display: flex;
    align-items: center;
    justify-content: space-between;
}

.option-row * {
    max-width: 60%;
}

.done-button {
    display: flex;
    justify-content: center;
    padding-bottom: 8px;
}

.header-option {
    display: flex;
    justify-content: space-between;
    align-items: center;

    padding-top: 8px;
    padding-bottom: 8px;
    width: 100%;
    border-left: 2px solid #888888;
}

.header-option * {
    color: #ffffff;
}

.material-icons {
    color: #ffffff;
    margin-left: 16px;
    margin-right: 16px;
    cursor: pointer;
}

.hidden {
    background-color: var(--gray);
}

.column-settings {
    width: 30%;
}

.button-row {
    display: flex;
    justify-content: space-evenly;
    align-items: center;
}
</style>
