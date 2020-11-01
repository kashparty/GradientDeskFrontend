<template>
    <div class="page" v-if="datasetInfo">
        <div class="workspace">
            <div class="settings">
                <div class="page-title no-select">
                    Configure your "{{ datasetInfo.name }}" dataset
                </div>
                <div class="option-row">
                    Read column headings from file
                    <CheckBox
                        v-model="readHeaders"
                        @input="parseData"
                        checked
                    ></CheckBox>
                </div>
            </div>
            <div class="table-wrapper">
                <table>
                    <tr>
                        <th v-for="(header, h) in headers" :key="h">
                            <div class="header-option no-select">
                                <div
                                    class="header-name"
                                    v-if="!isEditing(h)"
                                    @click="toggleEdit(h)"
                                >
                                    {{ header }}
                                </div>
                                <div v-else>
                                    <TextInput
                                        v-on:enter="toggleEdit(h)"
                                        placeholder="Enter to submit..."
                                        v-model="headers[h]"
                                        autofocus
                                    ></TextInput>
                                </div>
                                <div>
                                    <i
                                        class="material-icons header-icon"
                                        @click="hide(h)"
                                        :title="
                                            isHidden(h)
                                                ? 'Column excluded'
                                                : 'Column included'
                                        "
                                        >{{
                                            isHidden(h)
                                                ? "visibility_off"
                                                : "visibility"
                                        }}</i
                                    >
                                    <i
                                        class="material-icons header-icon"
                                        @click="makeNumeric(h)"
                                        :title="
                                            isNumeric(h)
                                                ? 'Numeric column'
                                                : 'Text column'
                                        "
                                        >{{
                                            isNumeric(h)
                                                ? "leaderboard"
                                                : "text_snippet"
                                        }}</i
                                    >
                                </div>
                            </div>
                        </th>
                    </tr>
                    <tr v-for="r in rows" :key="r">
                        <td
                            v-for="(col, c) in parsedData"
                            :class="{ hidden: isHidden(c) }"
                            :key="c"
                        >
                            {{ isHidden(c) ? "" : col[r - 1] }}
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
import TextInput from "./ui/TextInput";
export default {
    name: "EditDataset",
    components: {
        CheckBox,
        TextInput,
    },
    data() {
        return {
            datasetInfo: null,
            rawData: "",
            headers: [],
            parsedData: [],
            rows: 0,
            readHeaders: true,
            hiddenCols: [],
            numericCols: [],
            editingCol: -1,
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
                    });
                }
            });
        },
        parseData() {
            this.headers = [];
            let parsedData = [];

            let delimiter = null;
            if (this.datasetInfo.filetype == "csv") {
                delimiter = ",";
            } else if (this.datasetInfo.filetype == "tsv") {
                delimiter = "\t";
            }

            let lines = this.rawData.split("\n");
            while (lines[lines.length - 1] == "") lines.pop();

            this.rows = Math.min(lines.length, 50);
            let cols = 0;
            let firstValues = lines[0].split(delimiter).map((v) => v.trim());
            for (let v = 0; v < firstValues.length; v++) {
                parsedData.push([]);
                this.headers.push(`Column ${v + 1}`);
                cols++;
            }

            if (this.readHeaders) {
                this.headers = firstValues;
                lines.shift();
            }

            for (let l = 0; l < this.rows; l++) {
                let values = lines[l].split(delimiter);
                for (let v = 0; v < cols; v++) {
                    parsedData[v].push(values[v].trim());
                }
            }

            this.parsedData = parsedData;
        },
        hide(colNumber) {
            let index = this.hiddenCols.indexOf(colNumber);
            if (index >= 0) {
                // Column is already hidden, so un-hide it
                this.hiddenCols.splice(index, 1);
            } else {
                this.hiddenCols.push(colNumber);
            }
        },
        isHidden(colNumber) {
            return this.hiddenCols.indexOf(colNumber) >= 0;
        },
        makeNumeric(colNumber) {
            let index = this.numericCols.indexOf(colNumber);
            if (index >= 0) {
                // Column is already numeric, so revert it
                this.numericCols.splice(index, 1);
            } else {
                this.numericCols.push(colNumber);
            }
        },
        isNumeric(colNumber) {
            return this.numericCols.indexOf(colNumber) >= 0;
        },
        isEditing(colNumber) {
            return this.editingCol >= 0 && this.editingCol == colNumber;
        },
        toggleEdit(colNumber) {
            if (this.editingCol == colNumber) {
                this.editingCol = -1;
                if (this.headers[colNumber] == "") {
                    this.headers[colNumber] = `Column ${colNumber + 1}`;
                }
            }
            else {
                this.editingCol = colNumber;
            }
        }
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
    padding: 8px;
    display: flex;
    align-items: center;
    justify-content: space-between;
}

.header-option {
    display: flex;
    justify-content: space-around;
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
    margin-left: 8px;
    cursor: pointer;
}

.hidden {
    background-color: var(--gray);
}

.header-name {
    cursor: pointer;
}
</style>
