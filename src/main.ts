import './index.css';

import { createApp } from "vue";
import { createMemoryHistory, createRouter } from 'vue-router'

import IndexView from './pages/Index.vue'
import HostsView from './pages/Hosts.vue'

const routes = [
  { path: '/', component: IndexView },
  { path: '/hosts', component: HostsView },
]

const router = createRouter({
  history: createMemoryHistory(),
  routes,
})

import App from "./App.vue";

createApp(App).use(router).mount("#app");
