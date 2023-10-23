<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import { ComboChart } from "@carbon/charts-svelte";
    import "@carbon/charts-svelte/styles.css";
    import { onMount } from "svelte";

    let data: any;
    let options: any;
    onMount(async () => {
        //  初回読み込み
        fetchData();
        //  １分ごとにdata.jsを再読み込み
        setInterval(fetchData, 10000);
    });

    async function fetchData() {
        try {
            let responce: string[] = await invoke("fetch_spreadsheet_data", {
                fileName: "bar6.js",
            });
            //  データの変換
            let res_data = responce[0];
            data = JSON.parse(JSON.parse(res_data));

            let res_options = responce[1];
            options = JSON.parse(JSON.parse(res_options));
        } catch (error) {
            console.error("インヴォークエラー", error);
        }
    }
</script>

<div>
    {#if data}
        <ComboChart {data} {options} style="padding:1px;" />
    {:else}
        データ未受信
    {/if}
</div>
