<script setup lang="ts">
import { ArchiveX, Command, File, Inbox, Send, Trash2 } from 'lucide-vue-next'
import { h, ref } from 'vue'
import { RouterLink } from 'vue-router'
import {
  Sidebar,
  SidebarContent,
  SidebarGroup,
  SidebarGroupContent,
  SidebarHeader,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
  type SidebarProps,
  useSidebar,
} from '@/components/ui/sidebar'

const props = withDefaults(defineProps<SidebarProps>(), {
  collapsible: 'icon',
})
// This is sample data
const data = {
  user: {
    name: 'shadcn',
    email: 'm@example.com',
    avatar: '/avatars/shadcn.jpg',
  },
  navMain: [
    {
      title: 'Dashboards',
      url: '/',
      icon: Inbox,
      isActive: true,
    },
    {
      title: 'Hosts',
      url: '/hosts',
      icon: File,
      isActive: false,
    },
    {
      title: 'Sent',
      url: '#',
      icon: Send,
      isActive: false,
    },
    {
      title: 'Junk',
      url: '#',
      icon: ArchiveX,
      isActive: false,
    },
    {
      title: 'Trash',
      url: '#',
      icon: Trash2,
      isActive: false,
    },
  ],
  mails: [
    {
      name: 'William Smith',
      email: 'williamsmith@example.com',
      subject: 'Meeting Tomorrow',
      date: '09:34 AM',
      teaser:
        'Hi team, just a reminder about our meeting tomorrow at 10 AM.\nPlease come prepared with your project updates.',
    },
    {
      name: 'Alice Smith',
      email: 'alicesmith@example.com',
      subject: 'Re: Project Update',
      date: 'Yesterday',
      teaser:
        'Thanks for the update. The progress looks great so far.\nLet\'s schedule a call to discuss the next steps.',
    },
    {
      name: 'Bob Johnson',
      email: 'bobjohnson@example.com',
      subject: 'Weekend Plans',
      date: '2 days ago',
      teaser:
        'Hey everyone! I\'m thinking of organizing a team outing this weekend.\nWould you be interested in a hiking trip or a beach day?',
    },
    {
      name: 'Emily Davis',
      email: 'emilydavis@example.com',
      subject: 'Re: Question about Budget',
      date: '2 days ago',
      teaser:
        'I\'ve reviewed the budget numbers you sent over.\nCan we set up a quick call to discuss some potential adjustments?',
    },
    {
      name: 'Michael Wilson',
      email: 'michaelwilson@example.com',
      subject: 'Important Announcement',
      date: '1 week ago',
      teaser:
        'Please join us for an all-hands meeting this Friday at 3 PM.\nWe have some exciting news to share about the company\'s future.',
    },
    {
      name: 'Sarah Brown',
      email: 'sarahbrown@example.com',
      subject: 'Re: Feedback on Proposal',
      date: '1 week ago',
      teaser:
        'Thank you for sending over the proposal. I\'ve reviewed it and have some thoughts.\nCould we schedule a meeting to discuss my feedback in detail?',
    },
    {
      name: 'David Lee',
      email: 'davidlee@example.com',
      subject: 'New Project Idea',
      date: '1 week ago',
      teaser:
        'I\'ve been brainstorming and came up with an interesting project concept.\nDo you have time this week to discuss its potential impact and feasibility?',
    },
    {
      name: 'Olivia Wilson',
      email: 'oliviawilson@example.com',
      subject: 'Vacation Plans',
      date: '1 week ago',
      teaser:
        'Just a heads up that I\'ll be taking a two-week vacation next month.\nI\'ll make sure all my projects are up to date before I leave.',
    },
    {
      name: 'James Martin',
      email: 'jamesmartin@example.com',
      subject: 'Re: Conference Registration',
      date: '1 week ago',
      teaser:
        'I\'ve completed the registration for the upcoming tech conference.\nLet me know if you need any additional information from my end.',
    },
    {
      name: 'Sophia White',
      email: 'sophiawhite@example.com',
      subject: 'Team Dinner',
      date: '1 week ago',
      teaser:
        'To celebrate our recent project success, I\'d like to organize a team dinner.\nAre you available next Friday evening? Please let me know your preferences.',
    },
  ],
}
const activeItem = ref(data.navMain[0])
const mails = ref(data.mails)
const { setOpen } = useSidebar()
</script>
<template>
  <Sidebar class="overflow-hidden [&>[data-sidebar=sidebar]]:flex-row" v-bind="props">
    <!-- This is the first sidebar -->
    <!-- We disable collapsible and adjust width to icon. -->
    <!-- This will make the sidebar appear as icons. -->
    <Sidebar v-bind="props" collapsible="none" class="!w-[calc(var(--sidebar-width-icon)_+_1px)] border-r h-screen">
      <SidebarHeader>
        <SidebarMenu>
          <SidebarMenuItem>
            <SidebarMenuButton size="lg" as-child class="md:h-8 md:p-0">
              <a href="#">
                <div
                  class="flex aspect-square size-8 items-center justify-center rounded-lg bg-sidebar-foreground text-orange-500">
                  <Command class="size-4" />
                </div>
                <div class="grid flex-1 text-left text-sm leading-tight">
                  <span class="truncate font-semibold">Acme Inc</span>
                  <span class="truncate text-xs">Enterprise</span>
                </div>
              </a>
            </SidebarMenuButton>
          </SidebarMenuItem>
        </SidebarMenu>
      </SidebarHeader>
      <SidebarContent>
        <SidebarGroup>
          <SidebarGroupContent class="px-1.5 md:px-0">
            <SidebarMenu>
              <SidebarMenuItem v-for="item in data.navMain" :key="item.title">
                <SidebarMenuButton :tooltip="h('div', { hidden: false }, item.title)"
                  :is-active="activeItem.title === item.title" class="px-2.5 md:px-2" :as="RouterLink" :to="item.url"
                  @click="() => {
                    activeItem = item
                    const mail = data.mails.sort(() => Math.random() - 0.5)
                    mails = mail.slice(0, Math.max(5, Math.floor(Math.random() * 10) + 1))
                    setOpen(true)
                  }">

                  <component :is="item.icon" />
                  <span>{{ item.title }}</span>
                </SidebarMenuButton>
              </SidebarMenuItem>
            </SidebarMenu>
          </SidebarGroupContent>
        </SidebarGroup>
      </SidebarContent>
      <!-- <SidebarFooter>
        <NavUser :user="data.user" />
      </SidebarFooter> -->
    </Sidebar>
  </Sidebar>
</template>
