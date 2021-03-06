import Vue from "vue";
import Router from "vue-router";

import Home from "./components/Home";
import Login from "./components/Login";
import Projects from "./components/Projects";
import Datasets from "./components/Datasets";
import NewDataset from "./components/NewDataset";
import NewProject from "./components/NewProject";
import EditDataset from "./components/EditDataset";
import EditProject from "./components/EditProject";
import ViewDataset from "./components/ViewDataset";
import SelectDataset from "./components/SelectDataset";
import PrepareDataset from "./components/PrepareDataset";
import Account from "./components/Account";

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
            meta: {
                title: "Log In"
            }
        },
        {
            path: "/account",
            name: "Account",
            component: Account,
            meta: {
                title: "Account"
            }
        },
        {
            path: "/projects",
            name: "Projects",
            component: Projects,
            meta: {
                title: "Projects"
            }
        },
        {
            path: "/projects/new",
            name: "NewProject",
            component: NewProject,
            meta: {
                title: "New Project"
            }
        },
        {
            path: "/projects/edit/:id",
            name: "EditProject",
            component: EditProject,
            meta: {
                title: "Edit Project"
            }
        },
        {
            path: "/datasets",
            name: "Datasets",
            component: Datasets,
            meta: {
                title: "Datasets"
            }
        },
        {
            path: "/datasets/new",
            name: "NewDataset",
            component: NewDataset,
            meta: {
                title: "New Dataset"
            }
        },
        {
            path: "/datasets/edit/:id",
            name: "EditDataset",
            component: EditDataset,
            meta: {
                title: "Edit Dataset"
            }
        },
        {
            path: "/datasets/view/:id",
            name: "ViewDataset",
            component: ViewDataset,
            meta: {
                title: "View Dataset"
            }
        },
        {
            path: "/datasets/select/:id",
            name: "SelectDataset",
            component: SelectDataset,
            meta: {
                title: "Select Dataset"
            }
        },
        {
            path: "/datasets/prepare/:projectId/:datasetId",
            name: "PrepareDataset",
            component: PrepareDataset,
            meta: {
                title: "Prepare Dataset"
            }
        }
    ],
});

export default router;
