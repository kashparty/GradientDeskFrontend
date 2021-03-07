<template>
    <div class="page">
        <div class="search-container">
            <div class="search">
                <TextInput
                    v-model="searchQuery"
                    @input="updateSearch"
                    placeholder="Search datasets..."
                ></TextInput>
            </div>
        </div>
        <div class="datasets-list">
            <div
                v-for="dataset in filteredDatasets"
                :key="dataset.datasetId"
                class="dataset-card"
            >
                <div class="dataset-name no-select">{{ dataset.name }}</div>
                <div class="no-select">{{ dataset.description }}</div>
                <div class="button-row">
                    <Button
                        title="Select this dataset"
                        class="dataset-button"
                        @click="select(dataset.datasetId)"
                    >
                        <i class="material-icons">library_add_check</i>
                    </Button>
                    <Button
                        title="View this dataset"
                        class="dataset-button"
                        @click="view(dataset.datasetId)"
                    >
                        <i class="material-icons">visibility</i>
                    </Button>
                </div>
            </div>
        </div>
    </div>
</template>

<script>
import Button from "./ui/Button";
import TextInput from "./ui/TextInput";
export default {
    name: "SelectDataset",
    components: {
        Button,
        TextInput,
    },
    data() {
        return {
            datasetData: [],
            filteredDatasets: [],
            searchQuery: "",
        };
    },
    beforeCreate() {
        this.$store.commit(
            "redirect",
            `/datasets/select/${this.$route.params.id}`
        );
        if (!this.$store.state.authorized) {
            this.$router.replace("/login");
            return;
        }

        fetch("https://nnvis.herokuapp.com/dataset/all", {
            headers: {
                Authorization: this.$store.state.jwt,
            },
        }).then((response) => {
            if (response.ok) {
                response.json().then((json) => {
                    if (json.errors && json.errors.length != 0) {
                        this.$router.replace("/login");
                    } else {
                        this.datasetData = json.data;
                        this.filteredDatasets = this.datasetData;
                    }
                });
            }
        });
    },
    methods: {
        select(datasetId) {
            this.$router.push(
                `/datasets/prepare/${this.$route.params.id}/${datasetId}`
            );
        },
        view(datasetId) {
            this.$router.push(`/datasets/view/${datasetId}`);
        },
        updateSearch() {
            this.filteredDatasets = this.datasetData;
            if (this.searchQuery.length > 0) {
                this.filteredDatasets = this.datasetData.filter((dataset) => {
                    if (
                        dataset.name
                            .toLowerCase()
                            .includes(this.searchQuery.toLowerCase())
                    )
                        return true;
                    if (
                        dataset.description
                            .toLowerCase()
                            .includes(this.searchQuery.toLowerCase())
                    )
                        return true;
                    return false;
                });
            }
        },
    },
};
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
.page {
    padding: 16px;
    height: 100%;
}

.search-container {
    display: flex;
    justify-content: center;
}

.datasets-list {
    display: flex;
    flex-wrap: wrap;
    justify-content: center;
    align-content: flex-start;
}

.page-title {
    text-align: center;
    font-size: var(--big-font);
    margin-bottom: 24px;
    font-variation-settings: "wght" 600;
}

.dataset-name {
    font-size: var(--medium-font);
    font-variation-settings: "wght" 600;
    cursor: pointer;
    margin-top: 8px;
    margin-bottom: 8px;
}

.dataset-card {
    background-color: #ffffff;
    height: 200px;
    width: 400px;
    border-radius: 5px;
    border: 2px solid var(--dark-gray);

    display: flex;
    flex-direction: column;
    justify-content: space-around;
    align-items: center;

    margin: 8px;
    padding-left: 16px;
    padding-right: 16px;
}

.button-row {
    width: 100%;
    display: flex;
    justify-content: center;
}

.dataset-button {
    margin-left: 8px;
    margin-right: 8px;
}
</style>