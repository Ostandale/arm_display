<script lang="ts">
    import Chart1 from "./Chart_BarChart.svelte";
    import Chart2 from "./Chart_HoriBarChart.svelte";
    import Chart3 from "./Chart_FlootHoriBarChart.svelte";
</script>

<main>
    <div class="container2">
        <div class="custom">
            <Chart1 />
        </div>
        <div class="custom">
            <Chart2 />
        </div>
        <div class="custom2">
            <Chart3 />
        </div>
    </div>
</main>

<style>
    .container2 {
        display: flex;
        align-content: space-around;
        flex-direction: row;
        flex-wrap: wrap;
        justify-content: space-around;
        border: 1px solid red;
    }
    .custom {
        width: 1100px;
        height: 600px;
        border: 1px solid blue;
    }
    .custom2 {
        width: 2400px;
        height: 1500;
        border: 1px solid green;
    }
</style>
