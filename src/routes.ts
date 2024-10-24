import App from "./App.vue";
import {createRouter, createWebHashHistory} from "vue-router";
import Home from "./components/Home.vue";
import AnimalDetails from "./components/AnimalDetails.vue";
import AdoptionsAdmin from "./components/AdoptionsAdmin.vue";
import Square from "./components/Square.vue";
import UserProfile from "./components/UserProfile.vue";

const routes = [
    {path: '/', component: App},
    {path: '/home', component: Home},
    {path: '/animal/:id', component: AnimalDetails},
    {path: '/demo', component: UserProfile},
    {path: '/adoptions', component: AdoptionsAdmin},
    {path: '/square', component: Square},
];

export const router = createRouter({
    history: createWebHashHistory(),
    routes
});
