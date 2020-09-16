import Vue from "vue";
import VueRouter from "vue-router";
import Home from "../view/Home";
import UserTable from "../view/UserTable";
import TeamTable from "../view/TeamTable";
import PermanentBanTable from "../view/PermanentBanTable";

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
        component: UserTable // TODO: Create more abstract component to unify all these tables
    },
    {
        path: "/teams",
        name: "Teams",
        component: TeamTable
    },
    {
        path: "/permanent-bans",
        name: "Permanent Bans",
        component: PermanentBanTable
    }
];

const options = {
    routes
};

export default new VueRouter(options);
