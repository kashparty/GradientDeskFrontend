<template>
    <div class="page" v-if="datasetInfo">
        <div class="workspace">
            <div class="settings">
                <div class="column-settings">
                    <div class="option-row" @change="updateChart">
                        Visualisation mode:
                        <select v-model="visMode">
                            <option value="table">Table</option>
                            <option value="single">Single column</option>
                            <option value="correlate">Correlate columns</option>
                        </select>
                    </div>

                    <!--Visualisation-specific settings-->
                    <div
                        class="option-row"
                        v-if="visMode == 'single'"
                        @change="updateChart"
                    >
                        Column:
                        <select v-model="singleCol">
                            <option
                                v-for="(col, c) in columnData.filter(
                                    (c) => c.include
                                )"
                                :key="c"
                                :value="col.index"
                            >
                                {{ col.name }}
                            </option>
                        </select>
                    </div>

                    <div
                        class="option-row"
                        v-if="
                            visMode == 'single' &&
                            columnData[singleCol].type == 'text'
                        "
                        @change="updateChart"
                    >
                        Visualisation type:
                        <select v-model="singleTextType">
                            <option value="bar">Bar chart</option>
                            <option value="pie">Pie chart</option>
                        </select>
                    </div>

                    <div
                        class="option-row"
                        v-if="
                            visMode == 'single' &&
                            columnData[singleCol].type == 'number'
                        "
                        @change="updateChart"
                    >
                        Visualisation type:
                        <select v-model="singleNumberType">
                            <option value="distrib">Distribution</option>
                            <option value="boxplot">Box plot</option>
                        </select>
                    </div>

                    <div
                        class="option-row"
                        v-if="visMode == 'correlate'"
                        @change="updateChart"
                    >
                        X-axis:
                        <select v-model="correlateX">
                            <option
                                v-for="(col, c) in columnData.filter(
                                    (c) => c.include && c.type == 'number'
                                )"
                                :key="c"
                                :value="col.index"
                            >
                                {{ col.name }}
                            </option>
                        </select>
                    </div>

                    <div
                        class="option-row"
                        v-if="visMode == 'correlate'"
                        @change="updateChart"
                    >
                        Y-axis:
                        <select v-model="correlateY">
                            <option
                                v-for="(col, c) in columnData.filter(
                                    (c) => c.include && c.type == 'number'
                                )"
                                :key="c"
                                :value="col.index"
                            >
                                {{ col.name }}
                            </option>
                        </select>
                    </div>

                    <div
                        class="option-row"
                        v-if="visMode == 'correlate'"
                        @change="updateChart"
                    >
                        Visualisation type:
                        <select v-model="correlateType">
                            <option value="scatter">Scatter</option>
                            <option value="boxplot">Box plots</option>
                        </select>
                    </div>
                </div>
            </div>
            <Plotly
                v-if="visMode != 'table'"
                class="chart"
                :key="updateKey"
                :data="chartData"
                :layout="chartLayout"
                :display-mode-bar="false"
            ></Plotly>
            <div class="table-wrapper" v-else>
                <table>
                    <tr>
                        <!--Column headers, including edit icons-->
                        <th
                            v-for="(col, c) in columnData.filter(
                                (c) => c.include
                            )"
                            :key="c"
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
                                    @click="viewSingle(col.index)"
                                    >visibility</i
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
import { Plotly } from "vue-plotly";
export default {
    name: "ViewDataset",
    components: {
        Plotly,
    },
    data() {
        return {
            datasetInfo: null,
            rawData: "",

            columnData: [],
            parsedData: [],
            rows: 0,

            updatekey: false,
            visMode: "table",

            singleCol: 0,
            singleTextType: "bar",
            singleNumberType: "distrib",
            correlateType: "scatter",
            correlateX: 0,
            correlateY: 0,

            chartData: null,
            chartLayout: null,
        };
    },
    created() {
        this.getData();
    },
    beforeCreate() {
        if (!this.$store.state.authorized) {
            this.$store.commit(
                "redirect",
                `/datasets/view/${this.$route.params.id}`
            );
            this.$router.push("/login");
            return;
        }
    },
    methods: {
        getData() {
            fetch(`https://localhost:5001/dataset/${this.$route.params.id}`, {
                headers: {
                    Authorization: this.$store.state.jwt,
                },
            }).then((response) => {
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
            // Fetch the column configurations
            if (this.columnData.length == 0) {
                fetch(
                    `https://localhost:5001/dataset/${this.$route.params.id}/columns`,
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
                                this.columnData = json.data;
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

                                // Find first non-hidden column
                                let found = false;
                                let i = 0;
                                while (!found && i < cols) {
                                    if (this.columnData[i].include) {
                                        found = true;
                                        this.singleCol = i;
                                    }
                                    i++;
                                }

                                // Find first non-hidden numerical column
                                if (this.columnData[this.singleCol].type == "number") {
                                    this.correlateX = this.singleCol;
                                    this.correlateY = this.singleCol;
                                } else {
                                    found = false;
                                    while (!found && i < cols) {
                                        if (this.columnData[i].include && this.columnData[i].type == "number") {
                                            found = true;
                                            this.correlateX = i;
                                            this.correlateY = i;
                                        }
                                        i++;
                                    }
                                }

                                this.parsedData = parsedData;
                            }
                        });
                    }
                });
            }
        },
        viewSingle(index) {
            this.singleCol = index;
            this.visMode = "single";
            this.updateChart();
        },
        updateChart() {
            if (this.visMode == "single") {
                let column = this.parsedData[this.singleCol];
                if (this.columnData[this.singleCol].type == "text") {
                    let data = {};
                    for (let x of column) {
                        if (x in data) data[x]++;
                        else data[x] = 1;
                    }

                    if (this.singleTextType == "bar") {
                        this.chartData = [
                            {
                                x: Object.keys(data),
                                y: Object.values(data),
                                type: "bar",
                            },
                        ];
                    } else {
                        this.chartData = [
                            {
                                values: Object.values(data),
                                labels: Object.keys(data),
                                type: "pie",
                            },
                        ];
                    }
                } else {
                    if (this.singleNumberType == "distrib") {
                        this.chartData = [
                            {
                                x: column.map((x) => Number(x)),
                                type: "histogram",
                            },
                        ];
                    } else {
                        this.chartData = [
                            {
                                x: column.map((x) => Number(x)),
                                name: this.columnData[this.singleCol].name,
                                type: "box",
                            },
                        ];
                    }
                }
            } else {
                let xColumn = this.parsedData[this.correlateX];
                let yColumn = this.parsedData[this.correlateY];

                if (this.correlateType == "scatter") {
                    this.chartData = [
                        {
                            x: xColumn,
                            y: yColumn,
                            mode: "markers",
                            type: "scatter",
                        },
                    ];
                } else {
                    this.chartData = [
                        {
                            x: yColumn.map((x) => Number(x)),
                            name: this.columnData[this.correlateY].name,
                            type: "box",
                        },
                        {
                            x: xColumn.map((x) => Number(x)),
                            name: this.columnData[this.correlateX].name,
                            type: "box",
                        },
                    ];
                }
            }
            this.updateKey = !this.updateKey;
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
    background-color: var(--blue);
    box-shadow: 0 -3px 0 1px var(--blue);
    z-index: 10;
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
}

.header-option {
    display: flex;
    justify-content: space-between;
    align-items: center;

    padding-top: 8px;
    padding-bottom: 8px;
    width: 100%;
    border-left: 2px solid var(--dark-blue);
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

.done-button {
    padding-bottom: 8px;
}

.chart {
    width: 80%;
    align-self: center;
}
</style>
