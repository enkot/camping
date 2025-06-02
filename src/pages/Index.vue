<script setup lang="ts">
import { computed, onMounted, ref, watch, customRef } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { useColorMode } from '@vueuse/core'

import { DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger } from '@/components/ui/dropdown-menu'
import Card from "@/components/Card.vue";
import DashboardSwitcher from "@/components/DashboardSwitcher.vue";
import { Button } from "@/components/ui/button";
import { Icon } from "@iconify/vue";
import { useAsyncStorageRef } from "@/composables";
import { Host, HostInfo, PingResult } from "@/types";
import AppHeader from '@/components/AppHeader.vue';


const mode = useColorMode()

const pingResults = ref<{ [host: string]: PingResult[] }>({});

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

async function ping() {
  await invoke("start_pinging", {
    ips: hosts.value?.map((item) => item.host) || [],
  });
}

listen("ping-result", (event: { payload: PingResult }) => {
  const { host } = event.payload;

  if (pingResults.value[host])
    pingResults.value[host].push(event.payload);
  else
    pingResults.value[host] = [event.payload];

  const info = hostsInfo.value.find(item => item.host === host);

  if (info) {
    info.continued = true;
  }
});

async function start(_hosts: string[]) {
  hostsInfo.value = hostsInfo.value
    .map(item => {
      const isPaused = _hosts.includes(item.host)
      
      return {
        ...item,
        paused: isPaused ? false : item.paused,
      };
    });

  await invoke("start_pinging", { ips: _hosts });
}

async function pause(_hosts: string[]) {
  hostsInfo.value = hostsInfo.value
    .map(item => {
      const isPaused = _hosts.includes(item.host)

      return {
        ...item,
        paused: isPaused || item.paused,
        continued: !isPaused && item.continued,
      }
    });

  await invoke("pause_pinging", { ips: _hosts});
}

function addHost(host: HostInfo) {
  console.log('host', host)
  hostsInfo.value = [...hostsInfo.value, host];
  
  start([host.host]);
}
</script>

<template>
  <AppHeader :breadcrumbs="['Dashboards']">
    <template #title>
      <DashboardSwitcher :hostsInfo @add-host="addHost" />
    </template>
    <template #right>
      <div class="ml-auto flex items-center space-x-2">
          <Button @click="start(hosts.map(item => item.host))" variant="outline">
            <Icon icon="radix-icons:play" />
            Start all
          </Button>
          <Button @click="pause(hosts.map(item => item.host))" variant="outline">
            <Icon icon="radix-icons:pause" />
            Pause all
          </Button>
          <DropdownMenu>
            <DropdownMenuTrigger as-child>
              <Button variant="outline">
                <Icon icon="radix-icons:moon"
                  class="h-[1.2rem] w-[1.2rem] rotate-0 scale-100 transition-all dark:-rotate-90 dark:scale-0" />
                <Icon icon="radix-icons:sun"
                  class="absolute h-[1.2rem] w-[1.2rem] rotate-90 scale-0 transition-all dark:rotate-0 dark:scale-100" />
                <span class="sr-only">Toggle theme</span>
              </Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent align="end">
              <DropdownMenuItem @click="mode = 'light'">
                Light
              </DropdownMenuItem>
              <DropdownMenuItem @click="mode = 'dark'">
                Dark
              </DropdownMenuItem>
              <DropdownMenuItem @click="mode = 'auto'">
                System
              </DropdownMenuItem>
            </DropdownMenuContent>
          </DropdownMenu>
        </div>
    </template>
  </AppHeader>
  <div class="flex-col md:flex">
    <!-- <div class="border-b-none">
      <div class="flex h-14 items-center px-2">
        <MainNav class="mx-6" />
      </div>
    </div> -->
    <div class="grid auto-rows-min gap-2 px-2 sm:grid-cols-3 mt-2">
      <Card v-for="host in [...hosts, ...hosts, ...hosts]" v-bind="host" @start="start([host.host])" @pause="pause([host.host])" />
    </div>
  </div>
</template>
