import App from "./App.vue";
import {createRouter, createWebHashHistory} from "vue-router";
import Home from "./components/Home.vue";
import AnimalDetails from "./components/AnimalDetails.vue";
import PostAnimal from "./components/PostAnimal.vue";

const routes = [
    {path: '/', component: App},
    {path: '/home', component: Home},
    {path: '/animal/:id', component: AnimalDetails},
    {path: '/demo', component: PostAnimal},
];

export const router = createRouter({
    history: createWebHashHistory(),
    routes
});
