<script lang="ts">
	import { onMount } from 'svelte'
	import init, { make_nes } from 'hello'

	onMount(async () => {
		await init()
		// send canvas id to wasm
		make_nes('wasm_canvas')
	})

</script>

<main>
	<h1 class="text-2xl font-bold">wasm playground</h1>
  <div id="wasm" class="bg-orange-400">
		<canvas id="wasm_canvas" class="w-200 h-200 mx-auto" />
	</div>
</main>

<style>
  :root {
    font-family: 'Helvetica Neue', sans-serif;
  }
</style>
