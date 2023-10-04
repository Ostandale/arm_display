<script lang="ts" src="https://apis.google.com/js/platform.js" async defer>
  import Greet from "./lib/Greet.svelte";
</script>

<meta
  name="google-signin-client_id"
  content="630326421714-66iri050l6psed860ut27nos2p1g1ggi.apps.googleusercontent.com"
/>
<main class="container">
  <div class="g-signin2" data-onsuccess="onSignIn" />

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
    <h3>TODOリスト</h3>
    あんなことやこんなこと１ あんなことやこんなこと２ あんなことやこんなこと３ あんなことやこんなこと４
    あんなことやこんなこと５ あんなことやこんなこと６ あんなことやこんなこと７
  </div>
  <p>Click on the Tauri, Vite, and Svelte logos to learn more.</p>

  <div class="row">
    <Greet />
  </div>
</main>

<style>
  .logo.vite:hover {
    filter: drop-shadow(0 0 2em #747bff);
  }

  .logo.svelte:hover {
    filter: drop-shadow(0 0 2em #ff3e00);
  }
</style>
