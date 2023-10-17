<script lang="ts">
    import { BulletChart } from "@carbon/charts-svelte";
    import "@carbon/charts-svelte/styles.css";
    import options from "./options_bar4.js";
    import data from "./data_bar4.js";
</script>

<div>
    <BulletChart {data} {options} style="padding:1px;" />
</div>
