import { computed, reactive, ref } from "vue";
import { defineStore } from "pinia";
import { Dashboard, Host, HostInfo, PingResult } from "@/types";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

export const useStore = defineStore(
  "hosts",
  () => {
    const pingResults = reactive<{ [host: string]: PingResult[] }>({});
    const hostsInfo = ref<HostInfo[]>([]);
    const dashboards = ref<Dashboard[]>([]);

    const hosts = computed<Host[]>(() => {
      return hostsInfo.value.map((item) => {
        const results = pingResults[item.host] || [];

        console.log("item.paused", item.paused);

        return {
          ...item,
          pingResults: results,
          lastPing: results[results.length - 1],
        };
      });
    });

    listen("ping-result", (event: { payload: PingResult }) => {
      const { host } = event.payload;

      if (pingResults[host]) pingResults[host].push(event.payload);
      else pingResults[host] = [event.payload];

      const info = hostsInfo.value.find((item) => item.host === host);

      if (info) {
        // info.paused = false;
        info.continued = true;
      }
    });

    async function start(_hosts: string[]) {
      hostsInfo.value = hostsInfo.value.map((item) => {
        const isPaused = _hosts.includes(item.host);

        return {
          ...item,
          paused: isPaused ? false : item.paused,
        };
      });

      await invoke("start_pinging", { ips: _hosts });
    }

    async function pause(_hosts: string[]) {
      hostsInfo.value = hostsInfo.value.map((item) => {
        const isPaused = _hosts.includes(item.host);

        return {
          ...item,
          paused: isPaused || item.paused,
          continued: !isPaused && item.continued,
        };
      });

      await invoke("pause_pinging", { ips: _hosts });
    }

    async function ping() {
      await invoke("start_pinging", {
        ips: hosts.value?.map((item) => item.host) || [],
      });
    }

    function addHost(host: HostInfo) {
      hostsInfo.value = [...hostsInfo.value, host];

      start([host.host]);
    }

    function editHost(_host: string, host: HostInfo) {
      hostsInfo.value = hostsInfo.value.map((h) => {
        if (h.host === _host) return { ...h, ...host };

        return h;
      });

      invoke("pause_pinging", { ips: [_host] });
    }

    function addDashboard(dashboard: Dashboard) {
      dashboards.value = [...dashboards.value, dashboard];
    }

    function editDashboard(_dashboard: string, host: HostInfo) {
      dashboards.value = dashboards.value.map((d) => {
        if (d.name === _dashboard) return { ...d, ...host };

        return d;
      });
    }

    function addDashboardHost(dashboard: string, host: string) {
      dashboards.value = dashboards.value.map((d) => {
        if (d.name === dashboard) {
          return {
            ...d,
            hosts: [...d.hosts, host],
          };
        }
        return d;
      });
    }

    function removeDashboardHost(dashboard: string, host: string) {
      dashboards.value = dashboards.value.map((d) => {
        if (d.name === dashboard) {
          return {
            ...d,
            hosts: d.hosts.filter((h) => h !== host),
          };
        }
        return d;
      });
    }

    return {
      hostsInfo,
      hosts,
      dashboards,
      start,
      pause,
      ping,
      addHost,
      editHost,
      addDashboard,
      editDashboard,
    };
  },
  {
    tauri: {
      saveOnChange: true,
      saveStrategy: "debounce",
      saveInterval: 1000,
    },
  }
);
