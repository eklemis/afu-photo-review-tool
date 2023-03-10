<script>
	// @ts-nocheck
	import { ImageZoom } from '$lib/image-zoom.js';
	import { invoke } from '@tauri-apps/api/tauri';
	import { rotate90, rotate180, rotate270 } from '$lib/rotates';

	import { createEventDispatcher } from 'svelte';
	export let img_source = '';
	export let curr_identity = {
		child_id: '',
		roll_shot: ''
	};
	export let identity_image = {};
	const dispatch = createEventDispatcher();

	const options = {
		width: 400,
		zoomWidth: 500,
		offset: { vertical: 0, horizontal: 10 }
	};

	let curr_rot_deg = 0;

	const iz = ImageZoom(document.getElementById('img-container'), options);

	const getOcrInfo = async (src_path, deg) => {
		let ocr_info = await invoke('get_ocr_info', { srcPath: src_path, deg: deg });
		console.log('Ocr Info:', ocr_info);
		return ocr_info;
	};

	const dispatchClose = () => {
		dispatch('close', {});
	};
</script>

<section
	class="absolute w-screen h-screen flex items-center content-center z-50 bg-slate-400 left-0 top-0"
>
	<div class="relative w-[50%] min-h-full border flex-col">
		<img src={img_source} alt="" id="identity" />
		<div>
			<label for="child_id">Child Id</label>
			<input type="text" id="child_id" bind:value={curr_identity.child_id} class="border" />
			<label for="child_id">#Roll shot</label>
			<input type="text" id="roll_shot" bind:value={curr_identity.roll_shot} />
		</div>
		<div class="absolute bottom-0 flex gap-x-2 border">
			<button
				class="border rounded-md px-4 py-2 bg-white text-gray-800"
				on:click={() => rotate90('identity')}>rot 90</button
			>
			<button
				class="border rounded-md px-4 py-2 bg-white text-gray-800"
				on:click={() => rotate180('identity')}>rot 180</button
			>
			<button
				class="border rounded-md px-4 py-2 bg-white text-gray-800"
				on:click={() => {
					identity_image = rotate270('identity');
				}}>rot 270</button
			>
			<button
				class="border rounded-md px-4 py-2 bg-white text-gray-800"
				on:click={() =>
					setTimeout(() => {
						getOcrInfo(identity_image.path, curr_rot_deg);
					}, 0)}>get info</button
			>
		</div>
	</div>
	<div id="img-container" style="width: 400px">
		<img src={img_source} alt="Contain identity for current child" />
	</div>
	<button on:click={dispatchClose}>Close</button>
</section>
