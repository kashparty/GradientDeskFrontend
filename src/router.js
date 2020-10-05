import Vue from "vue";
import Router from "vue-router";

import Home from "./components/Home";
import Login from "./components/Login";
import Projects from "./components/Projects";
import Datasets from "./components/Datasets";
import NewDataset from "./components/NewDataset";
import NewProject from "./components/NewProject";
import EditDataset from "./components/EditDataset";

Vue.use(Router);

const router = new Router({
    mode: "history",
    routes: [
        {
            path: "/",
            name: "Home",
            component: Home,
        },
        {
            path: "/login",
            name: "Login",
            component: Login,
        },
        {
            path: "/projects",
            name: "Projects",
            component: Projects,
        },
        {
            path: "/projects/new",
            name: "NewProject",
            component: NewProject
        },
        {
            path: "/datasets",
            name: "Datasets",
            component: Datasets,
        },
        {
            path: "/datasets/new",
            name: "NewDataset",
            component: NewDataset
        },
        {
            path: "/datasets/edit/:id",
            name: "EditDataset",
            component: EditDataset
        }
    ],
});

export default router;
