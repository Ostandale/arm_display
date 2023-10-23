<script lang="ts">
    import Chart4 from "./Chart_BulletChart.svelte";
    import Chart5 from "./Chart_GaugeChart.svelte";
    import Chart6 from "./Chart_ComboChart.svelte";
    import ScrollNews from "./Scroll_news.svelte";
</script>

<main>
    <ScrollNews />
    <div class="container2">
        <div class="custom">
            <Chart4 />
        </div>
        <div class="custom11">
            <Chart5 />
        </div>
        <div class="custom2">
            <Chart6 />
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
        width: 1500px;
        height: 650px;
        border: 1px solid blue;
    }

    .custom11 {
        width: 650px;
        height: 650px;
        border: 1px solid blue;
    }
    .custom2 {
        width: 2400px;
        height: 1600;
        border: 1px solid green;
    }
</style>
