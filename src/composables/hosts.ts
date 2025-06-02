import { createGlobalState } from '@vueuse/core'
import { useAsyncStorageRef } from "@/composables";
import { Host, HostInfo, PingResult } from "@/types";
import { invoke } from "@tauri-apps/api/core";

export const useHosts = createGlobalState(() => {
    const hostsInfo = useAsyncStorageRef<HostInfo[]>("hosts", [], {
        onInit() {
            // hostsInfo.value = [
            //   {
            //     alias: "Ali",
            //     host: "203.119.104.37",
            //     paused: false,
            //   },
            //   {
            //     alias: "Google",
            //     host: "8.8.8.8",
            //     paused: false
            //   },
            // ];

            ping();
        },
    });

    const hosts = computed<Host[]>(() => {
        return hostsInfo.value.map((item) => {
            const results = pingResults.value[item.host] || [];

            return {
                ...item,
                pingResults: results,
                lastPing: results[results.length - 1],
            };
        })
    });

    return {
        async ping() {
            await invoke("start_pinging", {
                ips: hosts.value?.map((item) => item.host) || [],
            });
        }
    }
})