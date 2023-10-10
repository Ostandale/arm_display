<script lang="ts">
  import Screen1 from "./lib/Screen1.svelte";
  import Screen2 from "./lib/Screen2.svelte";
  import Screen3 from "./lib/Screen3.svelte";
  import Screen4 from "./lib/Screen4.svelte";
  import Screen5 from "./lib/Screen5.svelte";

  let showScreen = [true, false, false, false, false];

  //  配列の要素を呼び出される度に１つづつずらす
  function switchScreens() {
    let lastElement = showScreen.pop();
    if (lastElement !== undefined) {
      showScreen.unshift(lastElement);
    }
    showScreen = showScreen;
  }
  // ミリ秒で画面が変わる時間をカウント
  setInterval(switchScreens, 5000);
</script>

<head>
  <meta charset="UTF-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <script type="module" defer src="./index.js"></script>
</head>
<!-- <meta
  name="google-signin-client_id"
  content="630326421714-66iri050l6psed860ut27nos2p1g1ggi.apps.googleusercontent.com"
/> -->
<main class="container">
  {#if showScreen[0]}
    <div><Screen1 /></div>
  {/if}
  {#if showScreen[1]}
    <div><Screen2 /></div>
  {/if}
  {#if showScreen[2]}
    <div><Screen3 /></div>
  {/if}
  {#if showScreen[3]}
    <div><Screen4 /></div>
  {/if}
  {#if showScreen[4]}
    <div><Screen5 /></div>
  {/if}
  <!-- <div class="g-signin2" data-onsuccess="onSignIn" />

  <div>
    <h3>開発カレンダー</h3>
    <iframe
      src="https://calendar.google.com/calendar/embed?src=30e8c3932f6b64009e81d925b22c2eeeef12d183b2e73c6f2dab6f6ef52f0f9d%40group.calendar.google.com&ctz=Asia%2FTokyo"
      style="border: 0"
      width="1000"
      height="600"
      frameborder="0"
      scrolling="no"
    />
  </div>

  <div>
    <embed src="test.xlsx" width="1024" height="1024" />
  </div>

  <p>Click on the Tauri, Vite, and Svelte logos to learn more.</p>

  <div class="row">
    <Greet />
  </div> -->
</main>

<style>
  .container {
    width: 1200px;
    height: 1200px;
    margin: 5px auto;
  }
</style>
