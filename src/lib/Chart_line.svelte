<script>
    import { Line } from "svelte-chartjs";
    import { data } from "./data_line.js";

    import {
        Chart,
        Title,
        Tooltip,
        Legend,
        LineElement,
        LinearScale,
        PointElement,
        CategoryScale,
    } from "chart.js";

    Chart.register(
        Title,
        Tooltip,
        Legend,
        LineElement,
        LinearScale,
        PointElement,
        CategoryScale
    );
</script>

<Line {data} options={{ responsive: true }} />
