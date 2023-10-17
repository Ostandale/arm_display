<script lang="ts">
    interface ScrollNews {
        input_day: string;
        interval_time: number;
        news_type: string;
        news_source: string;
        last_display_time: string;
        message: string;
    }

    const scroll_news: ScrollNews[] = [
        {
            input_day: "2023-10-04",
            interval_time: 30,
            news_type: "aaa",
            news_source: "開発",
            last_display_time: "2023-10-20/10:24",
            message: "てきとーニュース",
        },
        {
            input_day: "2023-10-04",
            interval_time: 30,
            news_type: "aaa",
            news_source: "開発",
            last_display_time: "2023-10-20/10:24",
            message: "てきとーニュース2",
        },
    ];
</script>

<div class="scrolling-text-container">
    <div class="scrolling-text">{scroll_news[0].message}</div>
</div>

<style>
    .scrolling-text-container {
        overflow: hidden;
        white-space: nowrap;
    }

    .scrolling-text {
        display: inline-block;
        animation: scrollText 2s linear infinite;
    }

    @keyframes scrollText {
        0% {
            transform: translateX(100%);
        }
        100% {
            transform: translateX(-100%);
        }
    }
</style>
