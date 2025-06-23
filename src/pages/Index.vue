<script setup lang="ts">
import { computed, onMounted, ref, watch, customRef, useHost } from "vue";
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
import { useHosts } from "@/composables/hosts";
import { useStore } from "@/stores/hosts";


const mode = useColorMode()

const store = useStore()
</script>

<template>
  <AppHeader :breadcrumbs="['Dashboards']">
    <template #title>
      <DashboardSwitcher :hostsInfo="store.hosts" @add-host="store.addHost" />
    </template>
    <template #right>
      <div class="ml-auto flex items-center space-x-2">
        <Button @click="store.start(store.hosts.map(item => item.host))" variant="outline">
          <Icon icon="radix-icons:play" />
          Start all
        </Button>
        <Button @click="store.pause(store.hosts.map(item => item.host))" variant="outline">
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
      <Card v-for="host in [...store.hosts, ...store.hosts, ...store.hosts]" v-bind="host"
        @start="store.start([host.host])" @pause="store.pause([host.host])" />
    </div>
  </div>
</template>
