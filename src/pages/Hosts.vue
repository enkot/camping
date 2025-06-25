<script setup lang="ts">
import AppHeader from '@/components/AppHeader.vue';
import type {
  ColumnDef,
  ColumnFiltersState,
  ExpandedState,
  SortingState,
  VisibilityState,
} from '@tanstack/vue-table'
import {
  FlexRender,
  getCoreRowModel,
  getExpandedRowModel,
  getFilteredRowModel,
  getPaginationRowModel,
  getSortedRowModel,
  useVueTable,
} from '@tanstack/vue-table'
import { ArrowUpDown } from 'lucide-vue-next'
import { h, ref, toRef } from 'vue'
import { Icon } from "@iconify/vue";
import { valueUpdater } from '@/utils'

import { Button } from '@/components/ui/button'
import { Checkbox } from '@/components/ui/checkbox'
import { Badge } from '@/components/ui/badge';

import { Input } from '@/components/ui/input'
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table'
import DropdownAction from '@/components/DataTableDropDown.vue'
import { Host, PingResult } from "@/types";
import HostEditDialog from '@/components/HostEditDialog.vue';
import { useStore } from '@/stores/hosts';

const store = useStore()
const hostAddress = ref<string | undefined>()

const showDialog = ref(false)

const columns: ColumnDef<Host>[] = [
  {
    id: 'select',
    header: ({ table }) => h(Checkbox, {
      'modelValue': table.getIsAllPageRowsSelected() || (table.getIsSomePageRowsSelected() && 'indeterminate'),
      'onUpdate:modelValue': (value: any) => table.toggleAllPageRowsSelected(!!value),
      'ariaLabel': 'Select all',
    }),
    cell: ({ row }) => h(Checkbox, {
      'modelValue': row.getIsSelected(),
      'onUpdate:modelValue': (value: any) => row.toggleSelected(!!value),
      'ariaLabel': 'Select row',
    }),
    enableSorting: false,
    enableHiding: false,
  },
  {
    accessorKey: 'host',
    header: 'Host',
    cell: ({ row }) => h('div', { class: 'capitalize' }, row.getValue('host')),
  },
  {
    accessorKey: 'alias',
    header: ({ column }) => {
      return h(Button, {
        variant: 'ghost',
        onClick: () => column.toggleSorting(column.getIsSorted() === 'asc'),
      }, () => ['Alias', h(ArrowUpDown, { class: 'ml-2 h-4 w-4' })])
    },
    cell: ({ row }) => h('div', row.getValue('alias')),
  },
  {
    accessorKey: 'lastPing',
    header: () => h('div', 'Availability'),
    cell: ({ cell, row }) => {
      const paused = row.original.paused;
      const lastPing = cell.getValue() as PingResult;
      // return paused ? 'paused' : (status === 'success' ? 'up' : 'down');
      return h(Badge, {
        class: !lastPing || paused ? 'bg-gray-500/10 text-gray-800 dark:text-gray-300' : lastPing.status === 'success'
          ? 'bg-lime-400/70 dark:bg-lime-500/10 text-lime-800 dark:text-lime-500'
          : 'bg-red-400/70 dark:bg-red-500/10 text-red-800 dark:text-red-500'
      }, () => !lastPing || paused ? 'paused' : (lastPing.status === 'success' ? 'up' : 'down'))
    },
    // cell: ({ row }) => {
    //   const duration = Number.parseFloat(row.getValue('lastPing.duration'))

    //   return h('div', { class: 'text-right font-medium' }, duration ? 'Up' : 'Down')
    // },
  },
  {
    id: 'actions',
    enableHiding: false,
    cell: ({ row }) => {
      const host = row.original

      return h(DropdownAction, {
        host,
        onEdit: () => {
          hostAddress.value = host.host
          showDialog.value = true
        },
        onExpand: row.toggleExpanded,
      })
    },
  },
]

const sorting = ref<SortingState>([])
const columnFilters = ref<ColumnFiltersState>([])
const columnVisibility = ref<VisibilityState>({})
const rowSelection = ref({})
const expanded = ref<ExpandedState>({})

const table = useVueTable({
  data: toRef(() => store.hosts),
  columns,
  getCoreRowModel: getCoreRowModel(),
  getPaginationRowModel: getPaginationRowModel(),
  getSortedRowModel: getSortedRowModel(),
  getFilteredRowModel: getFilteredRowModel(),
  getExpandedRowModel: getExpandedRowModel(),
  onSortingChange: updaterOrValue => valueUpdater(updaterOrValue, sorting),
  onColumnFiltersChange: updaterOrValue => valueUpdater(updaterOrValue, columnFilters),
  onColumnVisibilityChange: updaterOrValue => valueUpdater(updaterOrValue, columnVisibility),
  onRowSelectionChange: updaterOrValue => valueUpdater(updaterOrValue, rowSelection),
  onExpandedChange: updaterOrValue => valueUpdater(updaterOrValue, expanded),
  state: {
    get sorting() { return sorting.value },
    get columnFilters() { return columnFilters.value },
    get columnVisibility() { return columnVisibility.value },
    get rowSelection() { return rowSelection.value },
    get expanded() { return expanded.value },
  },
})
</script>

<template>
  <AppHeader>
    <template #title>
      Hosts
    </template>
    <template #right>
      <Button variant="outline" @click="showDialog = true">
        <Icon icon="radix-icons:plus" />
        Add Host
      </Button>
      <HostEditDialog v-model="showDialog" :host="hostAddress" />
    </template>
  </AppHeader>
  <div class="w-full px-4">
    <div class="flex items-center py-4">
      <Input class="max-w-sm" placeholder="Filter hosts..."
        :model-value="table.getColumn('alias')?.getFilterValue() as string"
        @update:model-value=" table.getColumn('alias')?.setFilterValue($event)" />
    </div>
    <div class="rounded-md border">
      <Table>
        <TableHeader>
          <TableRow v-for="headerGroup in table.getHeaderGroups()" :key="headerGroup.id">
            <TableHead v-for="header in headerGroup.headers" :key="header.id">
              <FlexRender v-if="!header.isPlaceholder" :render="header.column.columnDef.header"
                :props="header.getContext()" />
            </TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          <template v-if="table.getRowModel().rows?.length">
            <template v-for="row in table.getRowModel().rows" :key="row.id">
              <TableRow :data-state="row.getIsSelected() && 'selected'">
                <TableCell v-for="cell in row.getVisibleCells()" :key="cell.id">
                  <FlexRender :render="cell.column.columnDef.cell" :props="cell.getContext()" />
                </TableCell>
              </TableRow>
              <TableRow v-if="row.getIsExpanded()">
                <TableCell :colspan="row.getAllCells().length">
                  {{ JSON.stringify(row.original) }}
                </TableCell>
              </TableRow>
            </template>
          </template>

          <TableRow v-else>
            <TableCell :colspan="columns.length" class="h-24 text-center">
              No results.
            </TableCell>
          </TableRow>
        </TableBody>
      </Table>
    </div>

    <div class="flex items-center justify-end space-x-2 py-4">
      <div class="flex-1 text-sm text-muted-foreground">
        {{ table.getFilteredSelectedRowModel().rows.length }} of
        {{ table.getFilteredRowModel().rows.length }} row(s) selected.
      </div>
      <div class="space-x-2">
        <Button variant="outline" size="sm" :disabled="!table.getCanPreviousPage()" @click="table.previousPage()">
          Previous
        </Button>
        <Button variant="outline" size="sm" :disabled="!table.getCanNextPage()" @click="table.nextPage()">
          Next
        </Button>
      </div>
    </div>
  </div>
</template>