<script>
	//@ts-nocheck
	import { mainDestPath } from '$lib/store';
	import { onDestroy } from 'svelte';

	let newMainDestPath = '';
	const unsub_mainDestPath = mainDestPath.subscribe((value) => (newMainDestPath = value));
	onDestroy(unsub_mainDestPath);
</script>

<section class="flex-col gap-y-4">
	<h1 class="text-3xl">Paths configuration</h1>
	<div class="flex flex-col gap-5">
		<div class="flex flex-col gap-y-1">
			<label for="dest_path">Destination base path</label>
			<input id="dest_path" type="text" bind:value={newMainDestPath} class="border w-full" />
		</div>
		<button
			on:click={() => mainDestPath.set(newMainDestPath)}
			class="border w-14 bg-blue-600 text-white rounded-md p-1">Save</button
		>
	</div>
</section>
