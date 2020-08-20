<template>
    <div class="page">
        <div class="login-box">
            <div class="login-title no-select">Log in or Sign up</div>
            <div class="login-inputs">
                <TextInput placeholder="Username"></TextInput>
                <TextInput placeholder="Password" password></TextInput>
            </div>
            <div class="login-buttons">
                <Button>Forgot password</Button>
                <Button highlight>Submit</Button>
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
            Button
        }
    }
</script>

<style scoped>
    .page {
        padding: 16px;
        height: 100%;
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
    }

    .login-box {
        display: flex;
        flex-direction: column;
        align-items: center;
    }

    .login-title {
        font-size: 32px;
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
</style>