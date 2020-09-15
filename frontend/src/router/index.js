import Vue from "vue";
import VueRouter from "vue-router";
import Home from "../view/Home";
import UserTable from "../view/UserTable";

Vue.use(VueRouter);

const routes = [
    {
        path: "",
        name: "Home",
        component: Home
    },
    {
        path: "/users",
        name: "Users",
        component: UserTable
    }
];

const options = {
    routes
};

export default new VueRouter(options);
