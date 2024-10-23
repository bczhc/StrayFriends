import App from "./App.vue";
import {createRouter, createWebHashHistory, createWebHistory} from "vue-router";
import Home from "./components/Home.vue";
import AdoptionRequest from "./components/AdoptionRequest.vue";

const routes = [
    {path: '/', component: App},
    {path: '/home', component: Home},
    {path: '/demo', component: AdoptionRequest},
];

export const router = createRouter({
    history: createWebHashHistory(),
    routes
});
