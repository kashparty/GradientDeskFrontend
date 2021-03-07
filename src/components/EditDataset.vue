<template>
    <div class="page" v-if="datasetInfo">
        <div class="workspace">
            <div class="settings">
                <!--Settings for whole table-->
                <div class="column-settings" v-if="editColNum === null">
                    <div class="option-row">
                        Dataset name
                        <TextInput
                            v-model="datasetInfo.name"
                            :placeholder="datasetInfo.name"
                            :initialValue="datasetInfo.name"
                            :maxlength="32"
                        ></TextInput>
                    </div>
                    <div class="option-row">
                        Dataset description
                        <TextInput
                            v-model="datasetInfo.description"
                            :placeholder="datasetInfo.description"
                            :initialValue="datasetInfo.description"
                            :maxlength="128"
                        ></TextInput>
                    </div>
                    <div class="option-row">
                        Read column headings from file
                        <CheckBox
                            v-model="datasetInfo.readHeaders"
                            @input="parseData"
                            :checked="datasetInfo.readHeaders"
                        ></CheckBox>
                    </div>
                    <div class="button-row">
                        <div class="done-button">
                            <Button @click="deleteDataset"
                                >Delete dataset</Button
                            >
                        </div>
                        <div class="done-button">
                            <Button @click="editTableDone" highlight
                                >Done editing</Button
                            >
                        </div>
                    </div>
                </div>

                <!--Column-specific settings, only shown when editing a column-->
                <div v-else class="column-settings">
                    <div class="option-row">
                        Include column?
                        <CheckBox
                            v-model="editColInclude"
                            @input="changeColInclude"
                            :checked="editColInclude"
                        ></CheckBox>
                    </div>
                    <div class="option-row">
                        Column name
                        <TextInput
                            :placeholder="editColName"
                            v-model="editColName"
                            @input="changeColName"
                            :maxlength="32"
                            :initialValue="editColName"
                        ></TextInput>
                    </div>
                    <div class="option-row">
                        Data type
                        <select @change="changeColType" v-model="editColType">
                            <option value="text">Text</option>
                            <option value="number">Number</option>
                        </select>
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
                        <th v-for="(column, c) in columnData" :key="c">
                            <div class="header-option">
                                <i class="material-icons no-select">
                                    {{
                                        column.type == "number"
                                            ? "scatter_plot"
                                            : "notes"
                                    }}
                                </i>
                                <div class="header-name">
                                    {{ column.name }}
                                </div>
                                <i
                                    class="material-icons no-select"
                                    @click="editCol(c)"
                                    >edit</i
                                >
                            </div>
                        </th>
                    </tr>
                    <tr v-for="r in rows" :key="r">
                        <!--Actual column data. Note col[r - 1] is used since Vue for loops start at 1.-->
                        <td
                            v-for="(col, c) in parsedData"
                            :class="{ hidden: !columnData[c].include }"
                            :key="c"
                        >
                            {{ columnData[c].include ? col[r - 1] : "" }}
                        </td>
                    </tr>
                </table>
            </div>
        </div>
    </div>
</template>

<script>
import firebase from "firebase/app";
import CheckBox from "./ui/CheckBox";
import Button from "./ui/Button";
import TextInput from "./ui/TextInput";
export default {
    name: "EditDataset",
    components: {
        CheckBox,
        Button,
        TextInput,
    },
    data() {
        return {
            datasetInfo: null,
            rawData: "",

            columnData: [],
            parsedData: [],
            rows: 0,
            updatingData: false,

            editColNum: null,
            editColName: null,
            editColType: null,
            editColInclude: true,
        };
    },
    created() {
        this.getData();
    },
    beforeCreate() {
        if (!this.$store.state.authorized) {
            this.$store.commit(
                "redirect",
                `/datasets/edit/${this.$route.params.id}`
            );
            this.$router.push("/login");
            return;
        }
    },
    methods: {
        getData() {
            fetch(`https://nnvis.herokuapp.com/dataset/${this.$route.params.id}`, {
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
            // Headers are the column names, parsedData is the data itself
            let headers = [];
            let parsedData = [];

            let delimiter = "";
            if (this.datasetInfo.filetype == "csv") {
                delimiter = ",";
            } else {
                delimiter = "\t";
            }

            let lines = this.rawData.split("\n");

            // Remove blank lines at the end of the file
            while (lines[lines.length - 1] == "") lines.pop();

            // Limit number of loaded rows to prevent slowing down page
            this.rows = Math.min(lines.length, 50);

            // Read first row of data to determine number of columns and to set up data array
            let cols = 0;
            let firstValues = lines[0].split(delimiter).map((v) => v.trim());
            for (let v = 0; v < firstValues.length; v++) {
                parsedData.push([]);
                headers.push(`Column ${v + 1}`);
                cols++;
            }

            // If reading headers from file, remove first row row and use those values as header names
            if (this.datasetInfo.readHeaders) {
                headers = firstValues;
                lines.shift();
            }

            // Read the rest of the data up to the maximum defined previously
            for (let l = 0; l < this.rows; l++) {
                let values = lines[l].split(delimiter);
                for (let v = 0; v < cols; v++) {
                    parsedData[v].push(values[v].trim());
                }
            }

            // Fetch (or create) the column configurations
            if (this.columnData.length == 0) {
                fetch(
                    `https://nnvis.herokuapp.com/dataset/${this.$route.params.id}/columns`,
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

                                    // Used later to ensure that we update columns rather than create them
                                    this.updatingData = true;
                                }

                                // If the length is still 0 then the columns are not in the database, so create them
                                if (this.columnData.length == 0) {
                                    for (let h = 0; h < headers.length; h++) {
                                        this.columnData.push({
                                            datasetId: this.datasetInfo
                                                .datasetId,
                                            name: headers[h],
                                            type: "text",
                                            include: true,
                                            index: h,
                                        });
                                    }
                                }
                            }
                        });
                    }
                });
            } else {
                // Don't add more columns, change existing columns
                for (let h = 0; h < headers.length; h++) {
                    this.columnData[h].name = headers[h];
                }
            }
            this.parsedData = parsedData;
        },
        editCol(colNumber) {
            let columnData = this.columnData[colNumber];
            this.editColName = columnData.name;
            this.editColType = columnData.type;
            this.editColInclude = columnData.include;
            this.editColNum = colNumber;
        },
        editColDone() {
            this.editColNum = null;
            this.editColName = "";
            this.editColType = "";
            this.editColInclude = "";
        },
        changeColInclude() {
            this.columnData[this.editColNum].include = this.editColInclude;
        },
        changeColName() {
            this.columnData[this.editColNum].name = this.editColName;
        },
        changeColType() {
            this.columnData[this.editColNum].type = this.editColType;
        },
        editTableDone() {
            fetch("https://nnvis.herokuapp.com/dataset", {
                method: "PUT",
                headers: {
                    "Content-Type": "application/json",
                    Authorization: this.$store.state.jwt,
                },
                body: JSON.stringify(this.datasetInfo),
            });
            fetch(`https://nnvis.herokuapp.com/dataset/columns`, {
                method: this.updatingData ? "PUT" : "POST",
                headers: {
                    "Content-Type": "application/json",
                    Authorization: this.$store.state.jwt,
                },
                body: JSON.stringify({
                    data: this.columnData,
                }),
            });
            this.$router.push("/datasets");
        },
        deleteDataset() {
            fetch(`https://nnvis.herokuapp.com/dataset/${this.$route.params.id}`, {
                method: "DELETE",
                headers: {
                    Authorization: this.$store.state.jwt,
                },
            }).then((response) => {
                if (response.ok) {
                    response.json().then((json) => {
                        if (json.errors && json.errors.length != 0) {
                            this.$emit("toast", json.errors[0]);
                        } else {
                            this.$router.push("/datasets");
                        }
                    });
                }
            });
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
    padding-bottom: 8px;
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

.button-row {
    display: flex;
    justify-content: space-evenly;
    align-items: center;
}
</style>
