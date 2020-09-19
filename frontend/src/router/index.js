import Vue from "vue";
import VueRouter from "vue-router";
import Home from "../view/Home";
import UserView from "../view/UserView";
import UserTable from "../view/UserTable";
import TeamView from "../view/TeamView";
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
        path: "/user/:uuid",
        name: "User",
        component: UserView
    },
    {
        path: "/users",
        name: "Users",
        component: UserTable
    },
    {
        path: "/team/:tag",
        name: "Team",
        component: TeamView
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
