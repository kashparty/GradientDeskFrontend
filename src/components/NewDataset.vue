<template>
    <div class="page">
        <input type="file" id="file-input" accept=".csv, .tsv" />
        <div class="form-box">
            <div class="help-text">
                You can import a file from GitHub (recommended), or upload a
                file from your computer (max 500KB).
            </div>
            <TextInput placeholder="Dataset name" v-model="name"></TextInput>
            <TextInput
                placeholder="Short description of the dataset (optional)"
                v-model="description"
            ></TextInput>
            <TextInput
                :placeholder="
                    uploading
                        ? 'Uploading ' + file.name
                        : 'GitHub file URL (if importing from GitHub)'
                "
                v-model="url"
                :disabled="uploading"
            ></TextInput>
            <div class="file-loaders">
                <Button @click="loadFile">{{
                    uploading ? "Cancel" : "Upload instead"
                }}</Button>
                <Button @click="submit" highlight>{{
                    uploading ? "Upload" : "Import from GitHub"
                }}</Button>
            </div>
        </div>
    </div>
</template>

<script>
import TextInput from "./ui/TextInput";
import Button from "./ui/Button";
import firebase from "firebase/app";

export default {
    name: "NewDataset",
    components: {
        TextInput,
        Button,
    },
    data() {
        return {
            name: "",
            description: "",
            url: "",
            file: null,
            uploading: false,
        };
    },
    beforeCreate() {
        if (!this.$store.state.authorized) {
            this.$store.commit("redirect", "/datasets/new");
            this.$router.push("/login");
            return;
        }
    },
    methods: {
        submit() {
            if (this.uploading) {
                this.upload();
                return;
            }

            let filetype = "";
            if (this.url.endsWith(".csv")) {
                filetype = "csv";
            } else if (this.url.endsWith(".tsv")) {
                filetype = "tsv";
            } else {
                this.$emit(
                    "toast",
                    "Invalid file type. Check that the URL points to a CSV or TSV file."
                );
                return;
            }

            fetch("https://localhost:5001/dataset", {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                    Authorization: this.$store.state.jwt,
                },
                body: JSON.stringify({
                    name: this.name,
                    description: this.description,
                    filetype: filetype,
                    url: this.url,
                }),
            }).then((response) => {
                if (response.ok) {
                    response.json().then((json) => {
                        if (json.errors && json.errors.length != 0) {
                            console.log(json.errors);
                            this.$emit("toast", json.errors.join(", "));
                        } else {
                            this.$router.push(`/datasets/edit/${json.data}`);
                        }
                    });
                }
            });
        },

        generateRandomString() {
            let array = new Uint8Array(16);
            window.crypto.getRandomValues(array);
            return Array.from(array, (x) =>
                x.toString(16).padStart(2, "0")
            ).join("");
        },

        loadFile() {
            if (this.uploading) {
                // Already loaded a file, so cancel loading
                this.uploading = false;
                return;
            }
            let fileInput = document.getElementById("file-input");
            fileInput.click();
            fileInput.addEventListener(
                "change",
                () => {
                    this.file = fileInput.files[0];
                    this.uploading = true;
                },
                false
            );
        },

        upload() {
            let filetype = "";
            if (this.file.name.endsWith(".csv")) {
                filetype = "csv";
            } else if (this.file.name.endsWith(".tsv")) {
                filetype = "tsv";
            } else {
                this.$emit(
                    "toast",
                    "Invalid file type. You need to upload a CSV or TSV file."
                );
                return;
            }

            if (this.file.size > 500 * 1000) {
                this.$emit(
                    "toast",
                    "Sorry, that file is too large. You can use GitHub for large files."
                );
                return;
            }

            let randomString = this.generateRandomString();

            let firebaseRef = firebase.storage().ref().child(randomString);
            firebaseRef.put(this.file).then(() => {
                fetch("https://localhost:5001/dataset", {
                    method: "POST",
                    headers: {
                        "Content-Type": "application/json",
                        Authorization: this.$store.state.jwt,
                    },
                    body: JSON.stringify({
                        name: this.name,
                        description: this.description,
                        filetype: filetype,
                        url: firebaseRef.fullPath,
                    }),
                }).then((response) => {
                    if (response.ok) {
                        response.json().then((json) => {
                            if (json.errors && json.errors.length != 0) {
                                console.log(json.errors);
                                this.$emit("toast", json.errors.join(", "));
                            } else {
                                this.$router.push(`datasets/edit/${json.data}`);
                            }
                        });
                    }
                });
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

.form-box {
    width: 400px;
    display: flex;
    flex-direction: column;
    align-items: center;
}

.file-loaders {
    width: 100%;
    display: flex;
    align-items: center;
    justify-content: space-between;
}

.help-text {
    max-width: 416px;
    text-align: center;
}

#file-input {
    display: none;
}
</style>
