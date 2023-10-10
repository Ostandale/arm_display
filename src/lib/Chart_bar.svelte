<script>
    import { Bar } from "svelte-chartjs";
    import { data } from "./data_bar.js";

    import {
        Chart,
        Title,
        Tooltip,
        Legend,
        BarElement,
        LinearScale,
        CategoryScale,
    } from "chart.js";

    Chart.register(
        Title,
        Tooltip,
        Legend,
        LinearScale,
        BarElement,
        CategoryScale
    );
</script>

<Bar {data} options={{ responsive: true }} />
