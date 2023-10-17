<script lang="ts">
    import { BarChartSimple } from "@carbon/charts-svelte";
    import "@carbon/charts-svelte/styles.css";
    import options from "./options_bar3.js";
    import data from "./data_bar3.js";
</script>

<div>
    <BarChartSimple {data} {options} style="padding:1px;" />
</div>
