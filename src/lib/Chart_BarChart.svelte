<script lang="ts">
    import { BarChartSimple } from "@carbon/charts-svelte";
    import "@carbon/charts-svelte/styles.css";
    import options from "./options_bar1.js";
    import data from "./data_bar1.js";
    import { ScaleTypes } from "@carbon/charts-svelte";
    import { ChartTheme } from "@carbon/charts-svelte";
</script>

<div>
    <BarChartSimple {data} {options} style="padding:1px;" />
</div>
