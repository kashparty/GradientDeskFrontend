<template>
    <div class="page">
        <div class="page-title no-select">
            {{
                projectData.length > 0
                    ? "Your projects"
                    : "Add your first project"
            }}
        </div>
        <div class="projects-list">
            <div class="new-project">
                <i class="material-icons add-icon no-select">add</i>
                <div class="no-select">New project</div>
            </div>
            <div
                v-for="project in projectData"
                :key="project.projectId"
                class="project-card"
            >
                <div class="project-name no-select">{{ project.name }}</div>
                <div class="no-select">{{ project.datasetId }}</div>
            </div>
        </div>
    </div>
</template>

<script>
export default {
    name: "Projects",
    data() {
        return {
            projectData: [],
        };
    },
    beforeCreate() {
        this.$store.commit("redirect", "/projects");
        if (!this.$store.state.authorized) {
            this.$router.replace("/login");
            return;
        }

        fetch("https://localhost:5001/project/all", {
            headers: {
                Authorization: this.$store.state.jwt,
            },
        }).then((response) => {
            if (response.ok) {
                response.json().then((json) => {
                    if (json.errors && json.errors.length != 0) {
                        this.$router.replace("/login");
                    } else {
                        this.projectData = json.data;
                    }
                });
            }
        });
    },
};
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
.page {
    padding: 16px;
    height: 100%;
}

.projects-list {
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

.project-name {
    font-size: var(--medium-font);
    font-variation-settings: "wght" 600;
    cursor: pointer;
}

.project-card {
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

.project-card:hover {
    border-color: var(--dark-blue);
}

.new-project {
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

.new-project:hover {
    border-color: var(--dark-blue);
}

.new-project:hover .add-icon {
    color: var(--dark-blue);
}

.add-icon {
    font-size: 48px;
}
</style>