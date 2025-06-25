import "./index.css";

import { createApp } from "vue";
import { createMemoryHistory, createRouter } from "vue-router";
import { createPinia } from "pinia";
import { createPlugin } from "@tauri-store/pinia";

import IndexView from "./pages/Index.vue";
import HostsView from "./pages/Hosts.vue";
import App from "./App.vue";

const routes = [
  { path: "/", component: IndexView },
  { path: "/hosts", component: HostsView },
];

const router = createRouter({
  history: createMemoryHistory(),
  routes,
});

const app = createApp(App);
const pinia = createPinia();

pinia.use(
  createPlugin({
    // autoStart: true,
  })
);

app.use(router);
app.use(pinia);
app.mount("#app");
