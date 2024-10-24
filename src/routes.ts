import App from "./App.vue";
import {createRouter, createWebHashHistory} from "vue-router";
import Home from "./components/Home.vue";
import AnimalDetails from "./components/AnimalDetails.vue";
import PostAnimal from "./components/PostAnimal.vue";
import AdoptionsAdmin from "./components/AdoptionsAdmin.vue";
import Square from "./components/Square.vue";

const routes = [
    {path: '/', component: App},
    {path: '/home', component: Home},
    {path: '/animal/:id', component: AnimalDetails},
    {path: '/demo', component: PostAnimal},
    {path: '/adoptions', component: AdoptionsAdmin},
    {path: '/square', component: Square},
];

export const router = createRouter({
    history: createWebHashHistory(),
    routes
});
