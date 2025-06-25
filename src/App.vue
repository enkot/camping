<script setup lang="ts">
import { RouterView } from "vue-router";
import {
  SidebarInset,
  SidebarProvider,
} from '@/components/ui/sidebar';
import AppSidebar from '@/components/AppSidebar.vue';
import { useStore } from "@/stores/hosts";

const store = useStore();
store.$tauri.start().then(() => {
  store.pause(store.hostsInfo.map((item) => item.host));
  // store.hostsInfo = store.hostsInfo.map((item) => {
  //   return {
  //     ...item,
  //     paused: true,
  //   };
  // });
});
</script>

<template>
  <SidebarProvider :style="{
    '--sidebar-width': '48px',
  }">
    <AppSidebar />
    <SidebarInset>
      <RouterView />
    </SidebarInset>
  </SidebarProvider>
</template>
