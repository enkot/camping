<script setup lang="ts">
import { ref } from 'vue';
import { RouterView } from "vue-router";
import {
  SidebarInset,
  SidebarProvider,
  SidebarTrigger,
} from '@/components/ui/sidebar';
import { useMenu } from '@/composables';
import AppSidebar from '@/components/AppSidebar.vue';

const opened = useMenu()
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
