<template>
    <div class="page">
        <div class="page-title no-select">
            {{
                datasetData.length > 0
                    ? "Your datasets"
                    : "Add your first dataset"
            }}
        </div>
        <div class="datasets-list">
            <div class="new-dataset" @click="newDataset">
                <i class="material-icons add-icon no-select">add</i>
                <div class="no-select">New dataset</div>
            </div>
            <div
                v-for="dataset in datasetData"
                :key="dataset.datasetId"
                @click="edit(dataset.datasetId)"
                class="dataset-card"
            >
                <div class="dataset-name no-select">{{ dataset.name }}</div>
                <div class="no-select">{{ dataset.description }}</div>
            </div>
        </div>
    </div>
</template>

<script>
export default {
    name: "Datasets",
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

        fetch("https://localhost:5001/dataset/all", {
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
            this.$router.push("datasets/new");
        },
        edit(datasetId) {
            this.$router.push(`datasets/edit/${datasetId}`);
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
}

.dataset-card {
    background-color: #ffffff;
    height: 200px;
    width: 400px;
    border-radius: 5px;
    border: 2px solid var(--dark-gray);

    display: flex;
    flex-direction: column;
    justify-content: space-evenly;
    align-items: center;
    cursor: pointer;

    margin: 8px;
}

.dataset-card:hover {
    border-color: var(--dark-blue);
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
</style>