<script setup lang="ts">
import { ref, onMounted, computed, watch } from 'vue';
import { useForm, FormContext } from 'vee-validate'
import { toTypedSchema } from '@vee-validate/zod'
import * as z from 'zod'

import {
    Dialog,
    DialogContent,
    DialogDescription,
    DialogFooter,
    DialogHeader,
    DialogTitle,
} from '@/components/ui/dialog'
import { vMaska } from "maska/vue"
import {
    Form,
    FormControl,
    FormField,
    FormItem,
    FormLabel,
    FormMessage,
} from '@/components/ui/form'
import { Input } from '@/components/ui/input'
import { Button } from '@/components/ui/button'

import { useHosts } from '@/composables/hosts'
import { Host } from '@/types';
import { useStore } from '@/stores/hosts';

const props = defineProps<{
    host?: string;
}>();

const show = defineModel<boolean>({ default: false })

const store = useStore()
const formRef = ref<FormContext | null>(null)

const formSchema = toTypedSchema(z.object({
    alias: z.string().min(2).max(50).optional(),
    host: z.string().ip().refine((host) => {
        // verify that ID exists in database
        return !store.hosts.filter(h => h.host !== props.host).some(h => h.host === host)
    }, {
        message: 'This host is already in use.'
    })
}))

watch(() => props.host, () => {
    const target = store.hosts.find(h => h.host === props.host)

    if (props.host && target) formRef.value?.setValues(target)
})

function onSubmit(values: any) {
    if (props.host)
        store.editHost(props.host, values as Host)
    else
        store.addHost(values as Host)

    show.value = false
}
</script>

<template>
    <Form v-slot="{ handleSubmit }" as="" ref="formRef" keep-values :validation-schema="formSchema">
        <Dialog v-model:open="show">
            <DialogContent>
                <DialogHeader>
                    <DialogTitle>{{ host ? 'Edit Host' : 'Add Host' }}</DialogTitle>
                    <DialogDescription>
                        {{ host ? 'Edit' : 'Add' }} host's address and alias.
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
                    <Button variant="outline" @click="show = false">
                        Cancel
                    </Button>
                    <Button type="submit" form="dialogForm">
                        Save
                    </Button>
                </DialogFooter>
            </DialogContent>
        </Dialog>
    </Form>
</template>