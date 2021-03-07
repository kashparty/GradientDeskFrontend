<template>
    <div class="page">
        <div class="projects-list">
            <div class="new-project" @click="newProject">
                <i class="material-icons add-icon no-select">add</i>
                <div class="no-select">New project</div>
            </div>
            <div
                v-for="project in projectData"
                :key="project.projectId"
                class="project-card"
            >
                <div class="project-name no-select">{{ project.name }}</div>
                <div class="no-select">{{ project.description }}</div>
                <div class="button-row">
                    <Button class="project-button" @click="edit(project.projectId)">
                        <i class="material-icons">edit</i>
                    </Button>
                    <Button class="project-button" @click="view(project.projectId)">
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
    name: "Projects",
    components: {
        Button
    },
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

        fetch("https://nnvis.herokuapp.com/project/all", {
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
    methods: {
        newProject() {
            this.$router.push("/projects/new");
        },
        edit(projectId) {
            this.$router.push(`/projects/edit/${projectId}`);
        },
        view(projectId) {
            this.$router.push(`/projects/view/${projectId}`)
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
    margin-top: 8px;
    margin-bottom: 8px;
}

.project-card {
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

    padding-left: 16px;
    padding-right: 16px;
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

.button-row {
    width: 100%;
    display: flex;
    justify-content: center;
}

.project-button {
    margin-left: 8px;
    margin-right: 8px;
}
</style>