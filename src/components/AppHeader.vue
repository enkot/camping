<script lang="ts" setup>
import {
    SidebarTrigger,
} from '@/components/ui/sidebar'
import {
    Breadcrumb,
    BreadcrumbList,
    BreadcrumbItem,
    BreadcrumbLink,
    BreadcrumbPage,
    BreadcrumbSeparator
} from '@/components/ui/breadcrumb'
import { Separator } from '@/components/ui/separator';

const { breadcrumbs = [] } = defineProps<{
    breadcrumbs?: string[]
}>()
</script>

<template>
    <header class="sticky top-0 flex border-b bg-background h-14 px-4 py-2 justify-between z-10">
        <div class="flex shrink-0 items-center gap-2">
            <SidebarTrigger class=" md:hidden -ml-1" />
            <Separator orientation="vertical" class=" md:hidden mr-2 h-4" />
            <Breadcrumb>
                <BreadcrumbList>
                    <template v-for="item in breadcrumbs">
                        <BreadcrumbItem class="hidden md:block">
                            <BreadcrumbLink href="#">
                                {{ item }}
                            </BreadcrumbLink>
                        </BreadcrumbItem>
                        <BreadcrumbSeparator class="hidden md:block" />
                    </template>
                    <BreadcrumbItem>
                        <BreadcrumbPage>
                            <slot name="title" />
                        </BreadcrumbPage>
                    </BreadcrumbItem>
                </BreadcrumbList>
            </Breadcrumb>
        </div>
        <slot name="right" />
    </header>
</template>