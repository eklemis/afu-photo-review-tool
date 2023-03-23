<script>
	//@ts-nocheck
	import WholeProgress from './whole_progress.svelte';
	import PgProgress from './pg_progress.svelte';
	import { get_all_photographers } from '$lib/rust_functions';
	import { onMount } from 'svelte';

	//choice = 1 (for all phtographer), 2 for specific photgrapher
	let choice = 1;
	let photographers = [];
	let photographer_id = 0;
	//special render setups for option 2
	let should_refresh = false;

	onMount(async () => {
		photographers = await get_all_photographers();
	});
</script>

<section class="mb-16 p-8 flex gap-x-2 min-h-[500px]">
	<div class="min-w-[350px]">
		<ul class="flex flex-col gap-y-1">
			<li>
				<button
					class={choice === 1
						? 'block w-full border rounded-sm p-2 bg-blue-500 text-white'
						: 'block w-full border rounded-sm p-2'}
					on:click={() => {
						choice = 1;
					}}>All Photographer</button
				>
			</li>
			<li>
				<button
					class={choice === 2
						? 'block w-full border rounded-sm p-2 bg-blue-500 text-white'
						: 'block w-full border rounded-sm p-2'}
					on:click={() => {
						choice = 2;
						should_refresh = true;
					}}>By Photographer</button
				>
			</li>
		</ul>
		{#if choice === 2}
			<div class="w-full mt-5 p-4">
				<label for="school" class="block text-sm font-medium leading-6 text-gray-900"
					>Pick a photographer</label
				>
				<select
					on:change={() => {
						should_refresh = true;
					}}
					bind:value={photographer_id}
					id="school"
					class="block w-full h-9 rounded-md border-0 py-1.5 px-2 text-gray-900 ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:outline-none focus:ring-indigo-600 sm:text-sm sm:leading-6"
				>
					{#each photographers as pg (pg.id)}
						<option value={pg.id}>{pg.name}</option>
					{/each}
				</select>
			</div>
		{/if}
	</div>
	{#if choice === 1}
		<WholeProgress />
	{:else}
		<PgProgress
			pg_id={photographer_id}
			refresh_now={should_refresh}
			on:refreshed={() => {
				should_refresh = false;
				console.log('refreshed done!');
			}}
		/>
	{/if}
</section>
