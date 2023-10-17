<script lang="ts">
    import { ComboChart } from "@carbon/charts-svelte";
    import "@carbon/charts-svelte/styles.css";
    import options from "./options_bar6.js";
    import data from "./data_bar6.js";
</script>

<div>
    <ComboChart {data} {options} style="padding:1px;" />
</div>
