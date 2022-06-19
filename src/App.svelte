<script>
	import { invoke } from '@tauri-apps/api/tauri'
  let count = 0
</script>

<main>
  <h1>{count}</h1>
  <button
    on:click={async () => {
      const res = await invoke('increse', {
        number: 1,
      })
      count = res.value
    }}>Increse</button
  >
</main>

<style>
  main {
    text-align: center;
    padding: 1em;
    max-width: 240px;
    margin: 0 auto;
  }

  h1 {
    color: #ff3e00;
    text-transform: uppercase;
    font-size: 4em;
    font-weight: 100;
  }

  @media (min-width: 640px) {
    main {
      max-width: none;
    }
  }
</style>
