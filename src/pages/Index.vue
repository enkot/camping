<script setup lang="ts">
import { useColorMode } from '@vueuse/core'
import { Swappable, Plugins } from '@shopify/draggable';

import { DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger } from '@/components/ui/dropdown-menu'
import Card from "@/components/Card.vue";
import DashboardSwitcher from "@/components/DashboardSwitcher.vue";
import { Button } from "@/components/ui/button";
import { Icon } from "@iconify/vue";
import { HostInfo } from "@/types";
import AppHeader from '@/components/AppHeader.vue';
import { useStore } from "@/stores/hosts";
import { onMounted } from 'vue';


const mode = useColorMode()

const store = useStore()

onMounted(() => {
  // Initialize draggable
  const containers = document.querySelectorAll('.draggable-container');
  const swappable = new Swappable(containers, {
    draggable: '.draggable-item',
    handle: '.draggable-handle',
    plugins: [Plugins.ResizeMirror]
  });
})
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
  <div class="flex h-full flex-col overflow-hidden md:h-screen -mt-14 pt-14">
    <!-- <div class="border-b-none">
      <div class="flex h-14 items-center px-2">
        <MainNav class="mx-6" />
      </div>
    </div> -->
    <div class="draggable-container grid gap-1 p-1 sm:grid-cols-3 flex-grow overflow-hidden">
      <Card v-for="host in [...store.hosts, ...store.hosts]" v-bind="host" @start="store.start([host.host])"
        @pause="store.pause([host.host])" handle-class="draggable-handle" class="flex-1 draggable-item" />
    </div>
  </div>
</template>

<style>
.draggable-source--is-dragging {
  opacity: 0;
  /* Hide original element during drag */
}
</style>
