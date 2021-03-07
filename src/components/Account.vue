<template>
    <div class="page" v-if="$store.state.authorized">
        <div class="form-box">
            <div class="title">{{$store.state.username}}</div>
            <div class="help-text">
                Your account is linked to<br />
                <em>{{ $store.state.email }}</em>
            </div>
            <TextInput
                placeholder="Username (leave blank to keep unchanged)"
                v-model="username"
                nocomplete
            ></TextInput>
            <TextInput
                placeholder="Password (leave blank to keep unchanged)"
                v-model="password"
                password
                nocomplete
            ></TextInput>
            <div class="button-row">
                <Button @click="deleteAccount">Delete account</Button>
                <Button @click="logout">Logout</Button>
                <Button @click="saveData" highlight>Save data</Button>
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
            username: "",
            password: "",
        };
    },
    beforeCreate() {
        if (!this.$store.state.authorized) {
            this.$store.commit("redirect", "/account");
            this.$router.push("/login");
            return;
        }
    },
    methods: {
        saveData() {
            if (this.password.length < 8 && this.password.length > 0) {
                this.$emit("toast", "Password shorter than 8 characters. Not changed.");
            }
            if (this.username.length == 0 && this.password.length < 8) {
                // Nothing is changed, so no need to send a request.
                return;
            }
            fetch("https://nnvis.herokuapp.com/user/edituser", {
                method: "PATCH",
                headers: {
                    "Content-Type": "application/json",
                    Authorization: this.$store.state.jwt,
                },
                body: JSON.stringify({
                    email: this.$store.state.email,
                    usernameChanged: this.username.length > 0,
                    passwordChanged: this.password.length >= 8,
                    username: this.username,
                    password: this.password,
                })
            }).then((response) => {
                if (response.ok && this.username.length > 0) {
                    this.$store.commit("changedUsername", this.username);
                }
            });
        },
        logout() {
            this.$store.commit("logout");
            this.$router.push("/");
        },
        deleteAccount() {
            fetch("https://nnvis.herokuapp.com/user", {
                method: "DELETE",
                headers: {
                    "Content-Type": "application/json",
                    Authorization: this.$store.state.jwt,
                },
                body: JSON.stringify({
                    email: this.$store.state.email,
                })
            }).then((response) => {
                if (response.ok) {
                    this.logout();
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
    justify-content: space-between;
}

.help-text {
    max-width: 416px;
    text-align: center;
    margin-bottom: 8px;
}

#file-input {
    display: none;
}
</style>
