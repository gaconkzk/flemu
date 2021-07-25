<script lang="ts">
	import { onMount } from 'svelte'
	import init, { make_nes } from 'hello'

	let canvas
	onMount(async () => {
		await init()
		// need both focus and tabindex for receive keyboard event
		canvas.setAttribute('tabindex','0')
		canvas.focus()

		// send canvas id to wasm
		make_nes('wasm_canvas')
	})

</script>

<main>
	<h1 class="text-2xl font-bold">wasm playground</h1>
  <div id="wasm" class="bg-orange-400" tabindex="0">
		<canvas id="wasm_canvas" class="w-200 h-200 mx-auto" bind:this={canvas}/>
	</div>
</main>

<style>
  :root {
    font-family: 'Helvetica Neue', sans-serif;
  }
</style>
