<template>
    <div class="page" v-if="datasetInfo">
        <div class="workspace">
            <div class="settings">
                <div class="page-title no-select">
                    Prepare your "{{ datasetInfo.name }}" dataset
                </div>
                <div class="option-row">
                    Read column headings from file
                    <TickBox v-model="readHeaders" @input="parseData" checked></TickBox>
                </div>
            </div>
            <div class="table-wrapper">
                <table>
                    <tr>
                        <th v-for="(header, h) in headers" :key="h">
                            {{ header }}
                        </th>
                    </tr>
                    <tr v-for="r in rows" :key="r">
                        <td v-for="(col, c) in parsedData" :key="c">
                            {{ col[r - 1] }}
                        </td>
                    </tr>
                </table>
            </div>
        </div>
    </div>
</template>

<script>
import firebase from "firebase/app";
import TickBox from "./ui/CheckBox";
export default {
    name: "EditDataset",
    components: {
        TickBox
    },
    data() {
        return {
            datasetInfo: null,
            rawData: "",
            headers: [],
            parsedData: [],
            rows: 0,
            readHeaders: true,
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

            this.rows = lines.length;
            let cols = 0;
            let firstValues = lines[0].split(delimiter);
            for (let v = 0; v < firstValues.length; v++) {
                parsedData.push([]);
                this.headers.push(`Column ${v+1}`);
                cols++;
            }

            if (this.readHeaders) {
                this.headers = firstValues;
                lines.shift();
            }

            for (let l = 0; l < lines.length; l++) {
                let values = lines[l].split(delimiter);
                for (let v = 0; v < cols; v++) {
                    parsedData[v].push(values[v]);
                }
            }
            this.parsedData = parsedData;
            this.dataLoaded = true;
        },
    },
};
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
.page {
    padding-top: 16px;
    padding-bottom: 16px;
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
}

.settings {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    min-width: 40vw;
}

.table-wrapper {
    overflow: auto;
    height: 100%;
    width: 100%;
    display: flex;
    justify-content: center;
}

table,
tr,
th,
td {
    border: 2px solid var(--gray);
    border-collapse: collapse;
}

th,
td {
    padding: 8px;
    min-width: 160px;
    text-align: center;
}

th {
    position: sticky;
    top: 2px;
    background-color: var(--blue);
    box-shadow: 0 -3px 0 1px var(--blue);
    color: #ffffff;
}

.option-row {
    padding: 8px;
    display: flex;
    align-items: center;
    justify-content: space-between;
}
</style>
