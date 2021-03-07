<template>
    <div class="page">
        <!-- <div class="page-title no-select">
            {{
                datasetData.length > 0
                    ? "Your datasets"
                    : "Add your first dataset"
            }}
        </div> -->
        <div class="datasets-list">
            <div class="new-dataset" @click="newDataset">
                <i class="material-icons add-icon no-select">add</i>
                <div class="no-select">New dataset</div>
            </div>
            <div
                v-for="dataset in datasetData"
                :key="dataset.datasetId"
                class="dataset-card"
            >
                <div class="dataset-name no-select">{{ dataset.name }}</div>
                <div class="no-select">{{ dataset.description }}</div>
                <div class="button-row">
                    <Button class="dataset-button" @click="edit(dataset.datasetId)">
                        <i class="material-icons">edit</i>
                    </Button>
                    <Button class="dataset-button" @click="view(dataset.datasetId)">
                        <i class="material-icons">visibility</i>
                    </Button>
                </div>
            </div>
        </div>
    </div>
</template>

<script>
import Button from "./ui/Button"; 
export default {
    name: "Datasets",
    components: {
        Button
    },
    data() {
        return {
            datasetData: [],
        };
    },
    beforeCreate() {
        this.$store.commit("redirect", "/datasets");
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
                    }
                });
            }
        });
    },
    methods: {
        newDataset() {
            this.$router.push("/datasets/new");
        },
        edit(datasetId) {
            this.$router.push(`/datasets/edit/${datasetId}`);
        },
        view(datasetId) {
            this.$router.push(`/datasets/view/${datasetId}`)
        }
    },
};
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
.page {
    padding: 16px;
    height: 100%;
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

.new-dataset {
    width: 400px;
    height: 200px;
    background-color: var(--gray);
    border: 2px solid var(--dark-gray);
    border-radius: 5px;
    margin: 8px;

    display: flex;
    justify-content: center;
    align-items: center;
    flex-direction: column;
    cursor: pointer;

    padding-left: 16px;
    padding-right: 16px;
}

.new-dataset:hover {
    border-color: var(--dark-blue);
}

.new-dataset:hover .add-icon {
    color: var(--dark-blue);
}

.add-icon {
    font-size: 48px;
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