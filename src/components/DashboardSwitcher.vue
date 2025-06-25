<script setup lang="ts">
import { ref } from 'vue'
import { Icon } from "@iconify/vue";

import { cn } from '@/lib/utils'
import {
    Avatar,
    AvatarFallback,
    AvatarImage,
} from '@/components/ui/avatar'
import { Button } from '@/components/ui/button'

import { Command, CommandEmpty, CommandGroup, CommandInput, CommandItem, CommandList, CommandSeparator } from '@/components/ui/command'
import {
    Dialog,
    DialogContent,
    DialogDescription,
    DialogFooter,
    DialogHeader,
    DialogTitle,
    DialogTrigger,
} from '@/components/ui/dialog'
import { Input } from '@/components/ui/input'
import {
    Popover,
    PopoverContent,
    PopoverTrigger,
} from '@/components/ui/popover'
import { vMaska } from "maska/vue"
import {
    Form,
    FormControl,
    FormField,
    FormItem,
    FormLabel,
    FormMessage,
} from '@/components/ui/form'
import { toTypedSchema } from '@vee-validate/zod'
import * as z from 'zod'
import { HostInfo } from '@/types';

const props = defineProps<{
    hostsInfo: HostInfo[]
}>()

const emit = defineEmits<{
    addHost: [HostInfo]
}>()

const groups = [
    {
        label: 'Dashboards',
        teams: [
            {
                label: 'Acme Inc.',
                value: 'acme-inc',
            },
            {
                label: 'Monsters Inc.',
                value: 'monsters',
            },
        ],
    },
]

type Team = (typeof groups)[number]['teams'][number]

const open = ref(false)
const showDialog = ref(false)
const selectedTeam = ref<Team>(groups[0].teams[0])

const formSchema = toTypedSchema(z.object({
    name: z.string().min(2).max(50)
}))

function onSubmit(values: any) {
    console.log('Form submitted!', values)
    emit('addHost', values)
    showDialog.value = false
}
</script>

<template>
    <Form v-slot="{ handleSubmit }" as="" keep-values :validation-schema="formSchema">
        <Dialog v-model:open="showDialog">
            <Popover v-model:open="open">
                <PopoverTrigger as-child>
                    <Button variant="outline" role="combobox" aria-expanded="open" aria-label="Select a team"
                        :class="cn('w-[200px] justify-between', $attrs.class ?? '')">
                        <Avatar class="mr-2 h-5 w-5">
                            <AvatarImage :src="`https://avatar.vercel.sh/${selectedTeam.value}.png`"
                                :alt="selectedTeam.label" />
                            <AvatarFallback>SC</AvatarFallback>
                        </Avatar>
                        {{ selectedTeam.label }}
                        <Icon icon="radix-icons:caret-sort" class="ml-auto h-4 w-4 shrink-0 opacity-50" />
                    </Button>
                </PopoverTrigger>
                <PopoverContent class="w-[200px] p-0">
                    <Command>
                        <CommandList>
                            <CommandInput placeholder="Search team..." />
                            <CommandEmpty>No team found.</CommandEmpty>
                            <CommandGroup v-for="group in groups" :key="group.label" :heading="group.label">
                                <CommandItem v-for="team in group.teams" :key="team.value" :value="team" class="text-sm"
                                    @select="() => {
                                        selectedTeam = team
                                        open = false
                                    }">
                                    <Avatar class="mr-2 h-5 w-5">
                                        <AvatarImage :src="`https://avatar.vercel.sh/${team.value}.png`"
                                            :alt="team.label" class="grayscale" />
                                        <AvatarFallback>SC</AvatarFallback>
                                    </Avatar>
                                    {{ team.label }}
                                    <Icon icon="radix-icons:check" :class="cn('ml-auto h-4 w-4',
                                        selectedTeam.value === team.value
                                            ? 'opacity-100'
                                            : 'opacity-0',
                                    )" />
                                </CommandItem>
                            </CommandGroup>
                        </CommandList>
                        <CommandSeparator />
                        <CommandList>
                            <CommandGroup>
                                <DialogTrigger as-child>
                                    <CommandItem value="create-team" @select="() => {
                                        open = false
                                        showDialog = true
                                    }">
                                        <Icon icon="radix-icons:plus-circled" class="mr-2 h-5 w-5" />
                                        Create Dashboard
                                    </CommandItem>
                                </DialogTrigger>
                            </CommandGroup>
                        </CommandList>
                    </Command>
                </PopoverContent>
            </Popover>
            <DialogContent>

                <DialogHeader>
                    <DialogTitle>Create dashboard</DialogTitle>
                    <DialogDescription>
                        Add a new dashboard to monitor specific hosts list.
                    </DialogDescription>
                </DialogHeader>
                <div>
                    <form id="dialogForm" @submit="handleSubmit($event, onSubmit)">
                        <div class="space-y-4 py-2 pb-4">
                            <div class="space-y-2">
                                <FormField v-slot="{ componentField }" name="alias">
                                    <FormItem>
                                        <FormLabel>Alias</FormLabel>
                                        <FormControl>
                                            <Input type="text" placeholder="Google" v-bind="componentField" />
                                        </FormControl>
                                        <FormMessage />
                                    </FormItem>
                                </FormField>
                            </div>
                            <div class="space-y-2">
                                <FormField v-slot="{ componentField }" name="host">
                                    <FormItem>
                                        <FormLabel>Host</FormLabel>
                                        <FormControl>
                                            <Input v-maska="'#00.#00.#00.#00'" data-maska-tokens="0:[0-9]:optional"
                                                placeholder="8.8.8.8" v-bind="componentField" />
                                        </FormControl>
                                        <FormMessage />
                                    </FormItem>
                                </FormField>
                            </div>
                        </div>
                    </form>
                </div>
                <DialogFooter>
                    <Button variant="outline" @click="showDialog = false" form="dialogForm">
                        Cancel
                    </Button>
                    <Button type="submit" form="dialogForm">
                        Continue
                    </Button>
                </DialogFooter>
            </DialogContent>
        </Dialog>
    </Form>
</template>