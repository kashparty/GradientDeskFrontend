<template>
    <div class="page">
        <div class="login-box">
            <div class="login-title no-select">Log in</div>
            <div class="login-inputs">
                <TextInput placeholder="Email" v-model="email"></TextInput>
                <TextInput
                    placeholder="Password"
                    password
                    v-model="password"
                ></TextInput>
            </div>
            <div class="login-buttons">
                <Button>Forgot password</Button>
                <Button @click="logIn" :loading="loggingIn" highlight
                    >Log in</Button
                >
            </div>
        </div>
        <div class="login-title no-select">or</div>
        <div class="login-box">
            <div class="login-title no-select">Sign up</div>
            <div class="login-inputs">
                <TextInput
                    placeholder="Username"
                    v-model="username"
                ></TextInput>
                <TextInput placeholder="Email" v-model="email"></TextInput>
                <TextInput
                    placeholder="Password"
                    password
                    v-model="password"
                ></TextInput>
            </div>
            <div class="signup-buttons">
                <Button @click="signUp" :loading="signingUp" highlight
                    >Sign up</Button
                >
            </div>
        </div>
    </div>
</template>

<script>
import TextInput from "./ui/TextInput";
import Button from "./ui/Button";
export default {
    name: "Login",
    components: {
        TextInput,
        Button,
    },
    data() {
        return {
            username: "",
            email: "",
            password: "",
            loggingIn: false,
            signingUp: false,
        };
    },
    methods: {
        logIn() {
            this.loggingIn = true;
            fetch("https://localhost:5001/user/login", {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                },
                body: JSON.stringify({
                    email: this.email,
                    password: this.password,
                }),
            }).then((response) => {
                if (response.ok) {
                    response.json().then((json) => {
                        if (json.errors && json.errors.length != 0) {
                            console.log(json.errors);
                            this.$emit("toast", json.errors.join(", "));
                        } else {
                            this.$store.commit("authorize", json.data);
                            this.$router.replace(
                                this.$store.state.redirectedFrom
                            );
                        }
                    });
                }

                this.loggingIn = false;
            });
        },
        signUp() {
            this.signingUp = true;
            fetch("https://localhost:5001/user", {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                },
                body: JSON.stringify({
                    username: this.username,
                    email: this.email,
                    password: this.password,
                }),
            }).then((response) => {
                if (response.ok) {
                    response.json().then((json) => {
                        if (json.errors && json.errors.length != 0) {
                            console.log(json.errors);
                            this.$emit("toast", json.errors.join(", "));
                        } else {
                            this.$store.commit("authorize", json.data);
                            console.log(json);
                            this.$router.replace(
                                this.$store.state.redirectedFrom
                            );
                        }
                    });
                }

                this.signingUp = false;
            });
        },
    },
};
</script>

<style scoped>
.page {
    padding: 16px;
    height: 100%;
    display: flex;
    justify-content: space-evenly;
    align-items: center;
}

.login-box {
    display: flex;
    flex-direction: column;
    align-items: center;
}

.login-title {
    font-size: var(--big-font);
    font-variation-settings: "wght" 600;
    margin-bottom: 24px;
    cursor: default;
}

.login-inputs {
    display: flex;
    flex-direction: column;
}

.login-buttons {
    width: 100%;
    display: flex;
    justify-content: space-between;
}

.signup-buttons {
    width: 100%;
    display: flex;
    justify-content: flex-end;
}
</style>