<script setup lang="ts">
import { ref, watch } from 'vue';
import { FormContext } from 'vee-validate'
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

import { useStore } from '@/stores/hosts';

const props = defineProps<{
    dashboard?: string;
}>();

const show = defineModel<boolean>({ default: false })

const store = useStore()
const formRef = ref<FormContext | null>(null)

const formSchema = toTypedSchema(z.object({
    name: z.string().min(2).max(50)
}))

watch(() => props.dashboard, () => {
    const target = store.dashboards.find(h => h.name === props.dashboard)

    if (props.dashboard && target) formRef.value?.setValues(target)
})

function onSubmit(values: any) {
    if (props.dashboard)
        store.editDashboard(props.dashboard, values)
    else
        store.addDashboard(values)

    show.value = false
}
</script>

<template>
    <Form v-slot="{ handleSubmit }" as="" ref="formRef" keep-values :validation-schema="formSchema">
        <Dialog v-model:open="show">
            <DialogContent>
                <DialogHeader>
                    <DialogTitle>{{ dashboard ? 'Edit Dashboard' : 'Add Dashboard' }}</DialogTitle>
                    <DialogDescription>
                        {{ dashboard ? 'Edit' : 'Add' }} dashboard's name.
                    </DialogDescription>
                </DialogHeader>
                <div>
                    <form id="dialogForm" @submit="handleSubmit($event, onSubmit)">
                        <div class="space-y-4 py-2 pb-4">
                            <div class="space-y-2">
                                <FormField v-slot="{ componentField }" name="name">
                                    <FormItem>
                                        <FormLabel>Name</FormLabel>
                                        <FormControl>
                                            <Input type="text" placeholder="My hosts" v-bind="componentField" />
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