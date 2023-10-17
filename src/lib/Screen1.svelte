<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
</script>

<div class="container2">
    <div class="custom1">
        <h3>開発カレンダー</h3>
        <iframe
            src="https://calendar.google.com/calendar/embed?src=30e8c3932f6b64009e81d925b22c2eeeef12d183b2e73c6f2dab6f6ef52f0f9d%40group.calendar.google.com&ctz=Asia%2FTokyo"
            title="開発カレンダー"
            style="border: 0"
            width="2000"
            height="1300"
            frameborder="0"
            scrolling="no"
        />
    </div>
</div>

<style>
    .container2 {
        display: flex;
        align-content: space-around;
        flex-direction: column;
        flex-wrap: wrap;
        justify-content: space-around;
        border: 1px solid red;
    }
</style>
