import App from "./App.vue";
import {createRouter, createWebHashHistory, createWebHistory} from "vue-router";
import Home from "./components/Home.vue";
import MyInfo from "./components/MyInfo.vue";

const routes = [
    {path: '/', component: App},
    {path: '/home', component: Home},
    {path: '/me', component: MyInfo},
];

export const router = createRouter({
    history: createWebHashHistory(),
    routes
});
