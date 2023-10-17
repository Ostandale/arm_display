<script lang="ts">
    import { GaugeChart } from "@carbon/charts-svelte";
    import "@carbon/charts-svelte/styles.css";
    import options from "../options_bar5.js";
    import data from "../data_bar5.js";
</script>

<div>
    <GaugeChart {data} {options} style="padding:1px;" />
</div>
