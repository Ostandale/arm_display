<script lang="ts">
    import { BarChartSimple } from "@carbon/charts-svelte";
    import "@carbon/charts-svelte/styles.css";
    import options from "../options_bar2.js";
    import data from "../data_bar2.js";
</script>

<div>
    <BarChartSimple {data} {options} style="padding:1px;" />
</div>
