<script>
	import { import_excel } from '$lib/rust_functions';
	let excel_path = '';
</script>

<h1>Photographers</h1>
<section class="flex flex-col gap-y-2">
	<h2>Import Photographers</h2>
	<div>
		<label for="excel_path">Path of excel file</label>
		<input type="text" class="border w-full" id="excel_path" bind:value={excel_path} />
	</div>
	<button class="border px-2 py-1 bg-red-400 text-gray-50" on:click={() => import_excel(excel_path)}
		>Import photographers</button
	>
</section>
