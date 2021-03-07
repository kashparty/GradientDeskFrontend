<template>
    <div class="page">
        <div class="page-inner">
            <div class="login-box">
                <div class="login-title no-select">Log in</div>
                <div class="login-inputs">
                    <TextInput placeholder="Email" v-model="email"></TextInput>
                    <TextInput
                        placeholder="Password"
                        password
                        v-model="password"
                        v-on:enter="logIn"
                    ></TextInput>
                </div>
                <div class="login-buttons">
                    <Button @click="forgotPassword">Forgot password</Button>
                    <Button @click="logIn" :loading="loggingIn" highlight
                        >Log in</Button
                    >
                </div>
            </div>
            <div class="login-title login-or no-select">or</div>
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
                        v-on:enter="signUp"
                    ></TextInput>
                </div>
                <div class="signup-buttons">
                    <Button @click="signUp" :loading="signingUp" highlight
                        >Sign up</Button
                    >
                </div>
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
            fetch("https://nnvis.herokuapp.com/user/login", {
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
            fetch("https://nnvis.herokuapp.com/user", {
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
        forgotPassword() {
            if (this.email.length != 0) {
                fetch("https://nnvis.herokuapp.com/user/resetpassword", {
                    method: "POST",
                    headers: {
                        "Content-Type": "application/json"
                    },
                    body: JSON.stringify({
                        email: this.email
                    }),
                }).then((response) => {
                    if (response.ok) {
                        response.json().then((json) => {
                            if (json.errors && json.errors.length != 0) {
                                this.$emit("toast", json.errors.join(", "));
                            }
                        });
                    }
                })
            }
        },
    },
};
</script>

<style scoped>
.page {
    padding: 16px;
    height: 100%;
    display: flex;
    justify-content: center;
    align-items: center;
}

.page-inner {
    width: 100%;
    display: flex;
    justify-content: space-evenly;
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

.login-or {
    align-self: center;
}

.login-inputs {
    width: 400px;
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