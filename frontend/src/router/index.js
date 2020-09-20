import Vue from "vue";
import VueRouter from "vue-router";
import HomeView from "../view/HomeView";
import UserView from "../view/UserView";
import UsersView from "../view/UsersView";
import TeamView from "../view/TeamView";
import TeamsView from "../view/TeamsView";
import PermanentBanView from "../view/PermanentBanView";
import PermanentBansView from "../view/PermanentBansView";

Vue.use(VueRouter);

const routes = [
    {
        path: "",
        name: "Home",
        component: HomeView
    },
    {
        path: "/user/:uuid",
        name: "User",
        component: UserView
    },
    {
        path: "/users",
        name: "Users",
        component: UsersView
    },
    {
        path: "/team/:tag",
        name: "Team",
        component: TeamView
    },
    {
        path: "/teams",
        name: "Teams",
        component: TeamsView
    },
    {
        path: "/permanent-ban/:uuid",
        name: "Permanent Ban",
        component: PermanentBanView
    },
    {
        path: "/permanent-bans",
        name: "Permanent Bans",
        component: PermanentBansView
    }
];

const options = {
    routes
};

export default new VueRouter(options);
