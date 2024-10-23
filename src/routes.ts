import App from "./App.vue";
import {createRouter, createWebHashHistory} from "vue-router";
import Home from "./components/Home.vue";
import AdoptionRequest from "./components/AdoptionRequest.vue";
import AnimalDetails from "./components/AnimalDetails.vue";

const routes = [
    {path: '/', component: App},
    {path: '/home', component: Home},
    {path: '/animal/:id', component: AnimalDetails},
    {path: '/demo', component: AdoptionRequest},
];

export const router = createRouter({
    history: createWebHashHistory(),
    routes
});
