<script lang="ts" setup>
import { computed, onMounted, ref } from "vue";
import { Line, Bar } from "vue-chartjs";
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
import { Button } from "@/components/ui/button";
import { PingResult } from "@/types";
import { useWindowSize } from "@vueuse/core";

const props = defineProps<{
  pingResults: PingResult[];
  lastPing: PingResult;
  alias: string;
  host: string;
  paused: boolean;
  continued?: boolean;
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

const { width } = useWindowSize();

const maxItems = computed(() => Math.round((width.value - 80) / 3 / 8));
const list = computed(() => {
  return Array.from({ length: maxItems.value }, (_, i) => {
    const latest = props.pingResults[props.pingResults.length - i - 1];
    const beforeLatest = props.pingResults[props.pingResults.length - i];

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

const options = {
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
    mode: "index", // Shows tooltip based on x-axis position
    intersect: false, // Allows tooltip to show even when not directly over a point
  },
  layout: {
    // padding: 0,
    padding: {
      top: 0,
      right: 0,
      bottom: -10,
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

const data = computed(() => {
  const max = Math.max(...list.value.map((item) => item.y || 0));
  const active = props.paused || !props.continued;

  return {
    datasets: [
      {
        type: "line",
        label: "Duration",
        borderColor: "rgba(0,0,0,0.8)",// '#1a2e05',
        // segment: {
        //   borderColor(ctx: any) {
        //     // const data = context.dataset.data;
        //     // const last = data[data.length - 1];

        //     return ctx.p1.parsed.y ? undefined : "#b91c1c";

        //     // return last?.isTimeout
        //     //   ? "oklch(44.4% 0.177 26.899)"
        //     //   : "oklch(53.2% 0.157 131.589)";
        //   },
        //   backgroundColor(ctx: any) {

        //     return ctx.p1.parsed.y ? undefined : "rgba(220, 38, 38, 0.5)";
        //   },
        //   spanGaps: true
        // },
        fill: "start", // Fill the area under the line
        data: list.value,
        borderWidth: 1,
        pointRadius: 0,
        borderRadius: 2,
        tension: 0.4,
        order: 2,
        clip: true,
      },
      {
        type: "bar",
        label: "Data Two",
        borderColor(context: any) {
          const data = context.dataset.data;
          const last = data[data.length - 1];

          return active ? "oklch(70.7% 0.022 261.325)" : "oklch(84.1% 0.238 128.85)";
        },
        backgroundColor(context: any) {
          const chart = context.chart;
          const { ctx, chartArea } = chart;
          const data = context.dataset.data;
          const last = data[data.length - 1];

          if (!chartArea) {
            // This case happens on initial chart load
            return;
          }
          const gradient = ctx.createLinearGradient(
            0,
            chartArea.bottom,
            0,
            chartArea.top
          );

          gradient.addColorStop(0, "#dc2626");
          gradient.addColorStop(1, "oklch(84.1% 0.238 128.85)");

          return gradient;
        },
        data: list.value.map((item) => {
          return {
            ...item,
            y: !item.y ? max + 20 : undefined,
          };
        }),
        barThickness: 8,
        pointRadius: 0,
        borderRadius: 2,
        borderWidth: 3,
        order: 1,
      },
    ],
  };
})
</script>

<template>
  <Card class="overflow-hidden border-0 pt-0 pb-0 border text-black" :class="paused || !continued ? 'bg-gray-400 border-gray-300' : lastPing?.status === 'timeout'
    ? 'bg-red-700 border-red-600'
    : 'bg-lime-400 border-lime-300'
    ">
    <CardContent class="relative px-3">
      <!-- <p class="text-sm text-muted-foreground">192.168.0.1</p> -->
      <p class="absolute opacity-80 top-1">{{ alias }}</p>
      <div class="flex mt-7 mb-1 gap-2 items-center">
        <Button size="sm" @click="() => paused ? $emit('start') : $emit('pause')">
          <Icon :icon="paused ? 'radix-icons:play' : 'radix-icons:pause'" class="w-4 h-4 font-bold" />
        </Button>
        <div class="text-4xl text-black">
          <template v-if="lastPing?.status === 'timeout'"> Timeout </template>
          <template v-else> {{ lastPing?.duration }}ms </template>
        </div>
      </div>

      <div class="relative -ml-6 -mr-4 h-16">
        <Line :data="data" :options="options" />
      </div>
    </CardContent>
    <CardHeader class="relative flex items-center h-8 justify-between px-3 text-xl bg-white/40">
      <span>
        {{ host }}
      </span>
      <Icon icon="radix-icons:edit" class="w-4 h-4 font-bold text-muted-foreground"></Icon>
    </CardHeader>
  </Card>
</template>
