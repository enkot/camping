import { ref } from "vue";
import { defineStore } from "pinia";
import { HostInfo, Dashboard } from "@/types";

export const useStore = defineStore("dashboards", () => {
  const dashboards = ref<Dashboard[]>([]);

  function addDashboard(dashboard: Dashboard) {
    dashboards.value = [...dashboards.value, dashboard];
  }

  function editDashboard(_dashboard: string, host: HostInfo) {
    dashboards.value = dashboards.value.map((d) => {
      if (d.name === _dashboard) return { ...d, ...host };

      return d;
    });
  }

  return {
    dashboards,
    addDashboard,
    editDashboard,
  };
});
