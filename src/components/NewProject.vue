<template>
    <div class="page">
        <div class="form-box">
            <div class="help-text">
                Give your new project a name and a description. 
                You'll configure the model and dataset later.
            </div>
            <TextInput placeholder="Project name" v-model="name"></TextInput>
            <TextInput
                placeholder="Short description of the project (optional)"
                v-model="description"
            ></TextInput>
            <div class="button-row">
                <Button @click="submit" highlight>Create</Button>
            </div>
        </div>
    </div>
</template>

<script>
import TextInput from "./ui/TextInput";
import Button from "./ui/Button";

export default {
    name: "NewDataset",
    components: {
        TextInput,
        Button,
    },
    data() {
        return {
            name: "",
            description: ""
        };
    },
    beforeCreate() {
        if (!this.$store.state.authorized) {
            this.$store.commit("redirect", "/projects/new");
            this.$router.push("/login");
            return;
        }
    },
    methods: {
        submit() {
            fetch("https://nnvis.herokuapp.com/project", {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                    Authorization: this.$store.state.jwt,
                },
                body: JSON.stringify({
                    name: this.name,
                    description: this.description
                }),
            }).then((response) => {
                if (response.ok) {
                    response.json().then((json) => {
                        if (json.errors && json.errors.length != 0) {
                            console.log(json.errors);
                            this.$emit("toast", json.errors.join(", "));
                        } else {
                            this.$router.push(`/projects/edit/${json.data}`);
                        }
                    })
                }
            });
        },
    },
};
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
.page {
    padding: 16px;
    height: 100%;
    display: flex;
    justify-content: space-evenly;
    align-items: center;
}

.title {
    text-align: center;
    font-size: var(--big-font);
    margin-bottom: 24px;
    font-variation-settings: "wght" 600;
}

.help-text {
    max-width: 416px;
    text-align: center;
}

.form-box {
    width: 400px;
    display: flex;
    flex-direction: column;
    align-items: center;
}

.button-row {
    width: 100%;
    display: flex;
    align-items: center;
    justify-content: flex-end;
}
</style>
