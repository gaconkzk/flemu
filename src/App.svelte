<script lang="ts">
	import init, { greet } from 'hello'

	console.log('call init')
	init().then((m) => {
		// greet('hihihi')
	})
</script>

<main>
  <div class="bg-orange-400 w-200 h-200 mx-auto">
		wasm playground
	</div>
</main>

<style>
  :root {
    font-family: 'Helvetica Neue', sans-serif;
  }
</style>
