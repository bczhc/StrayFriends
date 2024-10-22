import App from "./App.vue";
import {createRouter, createWebHashHistory, createWebHistory} from "vue-router";
import Home from "./components/Home.vue";

const routes = [
    {path: '/', component: App},
    {path: '/home', component: Home},
];

export const router = createRouter({
    history: createWebHashHistory(),
    routes
});
