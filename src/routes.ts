import App from "./App.vue";
import {createRouter, createWebHistory} from "vue-router";

const routes = [
    {path: '/', components: App},
];

export const router = createRouter({
    history: createWebHistory(),
    routes
});
