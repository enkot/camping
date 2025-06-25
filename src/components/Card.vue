<script lang="ts" setup>
import { computed, onMounted } from "vue";
import { Line } from "vue-chartjs";
import {
  Chart as ChartJS,
  CategoryScale,
  LogarithmicScale,
  LinearScale,
  PointElement,
  LineElement,
  BarElement,
  Title,
  Tooltip,
  Legend,
  Filler,
  TimeScale,
} from "chart.js";
import "chartjs-adapter-luxon";
import { Card, CardContent, CardHeader } from "@/components/ui/card";
import { Icon } from "@iconify/vue";

import { PingResult } from "@/types";
import { useColorMode } from "@vueuse/core";

const props = defineProps<{
  pingResults: PingResult[];
  lastPing: PingResult;
  alias: string;
  host: string;
  paused?: boolean;
  continued?: boolean;
  handleClass?: string;
}>();

defineEmits(["start", "pause"]);

ChartJS.register(
  CategoryScale,
  LinearScale,
  TimeScale,
  LogarithmicScale,
  PointElement,
  LineElement,
  BarElement,
  Title,
  Tooltip,
  Legend,
  Filler,
);

const mode = useColorMode()

const list = computed(() => {
  return Array.from({ length: 60 }, (_, i) => {
    const latest = props.pingResults[props.pingResults.length - i - 1];

    return (
      latest ? {
        x: new Date(latest.timestamp).toLocaleString(),
        y: latest.duration,
        status: latest.status,
      } : {
        x: `${i}`,
        y: undefined,
        status: "ok",
      }
    );
  }).reverse();
});

const options: any = {
  animation: false,
  responsive: true,
  maintainAspectRatio: false,
  plugins: {
    legend: {
      display: false,
    },
    tooltip: {
      filter(tooltipItem: any) {
        return tooltipItem.dataset.type !== 'bar';
      }
    }
  },
  interaction: {
    // mode: "index", // Shows tooltip based on x-axis position
    intersect: false, // Allows tooltip to show even when not directly over a point
  },
  layout: {
    // padding: 0,
    padding: {
      top: 0,
      right: 0,
      bottom: 0,
      left: 0,
    },
  },
  scales: {
    x: {
      display: true,
      beginAtZero: true,
      ticks: {
        display: false, // Hide x-axis labels
        padding: 0,
      },
      type: "category",
      // time: {
      //   unit: "second",
      // },
      grid: {
        display: false, // Hide vertical grid lines,
        drawOnChartArea: false,
        lineWidth: 0,
      },
      border: {
        display: false, // Hide the y-axis (ordinate) line
      },
    },
    y: {
      beginAtZero: false,
      ticks: {
        display: false, // Hide x-axis labels
        // padding: 10,
      },
      border: {
        display: false, // Hide the y-axis (ordinate) line
      },
      grid: {
        display: false, // Hide vertical grid lines,
        drawOnChartArea: false,
        lineWidth: 0,
      },
    },
  },
};

const data = computed<any>(() => {
  return {
    datasets: [
      {
        label: "Duration",
        borderColor: mode.value === 'light' ? 'oklch(40.5% 0.101 131.063)' : 'oklch(53.2% 0.157 131.589)',
        segment: {
          borderColor(ctx: any) {
            // const data = context.dataset.data;
            // const last = data[data.length - 1];

            return ctx.p1.parsed.y ? undefined : "oklch(39.6% 0.141 25.723)";

            // return last?.isTimeout
            //   ? "oklch(44.4% 0.177 26.899)"
            //   : "oklch(53.2% 0.157 131.589)";
          },
          // backgroundColor(ctx: any) {

          //   return ctx.p1.parsed.y ? undefined : "rgba(220, 38, 38, 1)";
          // },
          spanGaps: true
        },
        // fill: "start", // Fill the area under the line
        data: list.value,
        borderWidth: 2,
        pointRadius: 0,
        borderRadius: 2,
        tension: 0,
        order: 2,
        clip: true,
      },
      // {
      //   type: "bar",
      //   label: "Data Two",
      //   borderColor(context: any) {
      //     const data = context.dataset.data;
      //     const last = data[data.length - 1];

      //     return active ? "oklch(70.7% 0.022 261.325)" : "oklch(84.1% 0.238 128.85)";
      //   },
      //   backgroundColor(context: any) {
      //     const chart = context.chart;
      //     const { ctx, chartArea } = chart;
      //     const data = context.dataset.data;
      //     const last = data[data.length - 1];

      //     if (!chartArea) {
      //       // This case happens on initial chart load
      //       return;
      //     }
      //     const gradient = ctx.createLinearGradient(
      //       0,
      //       chartArea.bottom,
      //       0,
      //       chartArea.top
      //     );

      //     gradient.addColorStop(0, "#dc2626");
      //     gradient.addColorStop(1, "oklch(84.1% 0.238 128.85)");

      //     return gradient;
      //   },
      //   data: list.value.map((item) => {
      //     return {
      //       ...item,
      //       y: !item.y ? max + 20 : undefined,
      //     };
      //   }),
      //   barThickness: 8,
      //   pointRadius: 0,
      //   borderRadius: 2,
      //   borderWidth: 3,
      //   order: 1,
      // },
    ],
  };
})


</script>

<template>
  <Card class="overflow-hidden border-0 pt-0 pb-0 rounded-none border min-h-28" :class="paused || !continued ? '' : lastPing?.status === 'success'
    ? 'bg-lime-300/100 dark:bg-lime-950 border-lime-500 dark:border-lime-950'
    : 'bg-red-500/100 dark:bg-red-950 border-red-600 dark:border-red-950'
    ">
    <CardContent class="overflow-hidden relative px-3 flex-grow flex flex-col">
      <!-- <p class="text-sm text-muted-foreground">192.168.0.1</p> -->
      <div class="absolute right-2 top-2 h-8 w-8 flex items-center justify-center hover:bg-black/10 rounded"
        :class="handleClass">
        <Icon icon="radix-icons:drag-handle-dots-2" class="w-6 h-6 text-black/50 dark:text-white/50" />
      </div>
      <p class="text-primary/90 dark:text-muted-foreground mt-1.5 text-lg font-bold">{{ host }}</p>
      <div class="flex mt-1.5 mb-1 gap-2 items-center" :class="alias ? '' : ''">
        <!-- <Button size="sm" @click="() => paused ? $emit('start') : $emit('pause')" variant="secondary">
        </Button> -->
        <div
          class="text-2xl md:text-3xl xl:text-4xl p-1 px-2 rounded-md flex items-center gap-1 cursor-pointer shadow-xs"
          :class="paused || !continued ? 'bg-gray-500/10' : lastPing?.status === 'success'
            ? 'bg-lime-700/90 dark:bg-lime-500/10 text-white dark:text-lime-500 border-lime-200 dark:border-none'
            : 'bg-red-700/90 dark:bg-red-500/10 text-white dark:text-red-500 border-red-300/50 dark:border-none'
            " @click="() => paused ? $emit('start') : $emit('pause')">
          <Icon :icon="paused ? 'radix-icons:play' : 'radix-icons:pause'" class="w-8 h-8 font-bold" />
          <!-- <Play v-if="paused" />
          <Pause v-else /> -->
          <template v-if="lastPing?.status === 'timeout'"> Timeout </template>
          <template v-else> {{ lastPing?.duration }}ms </template>
        </div>
      </div>

      <div class="relative -ml-6 -mr-4 flex-grow overflow-hidden">
        <Line :data="data" :options="options" class="h-full" />
      </div>
    </CardContent>
    <CardHeader class="relative flex items-center h-8 justify-between px-3">
      <span :class="paused || !continued ? '' : lastPing?.status === 'success'
        ? 'text-lime-950 dark:text-lime-500'
        : 'text-red-950 dark:text-red-500'
        ">
        {{ alias }}
      </span>
      <!-- <Icon icon="radix-icons:edit" class="w-4 h-4 font-bold"></Icon> -->
    </CardHeader>
  </Card>
</template>
