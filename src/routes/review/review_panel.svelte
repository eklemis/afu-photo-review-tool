<script>
	// @ts-nocheck
	import { onMount, onDestroy } from 'svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	import { photo_paths, mainDestPath, selectedSchool } from '../../lib/store';
	import { get_higher_rotated_image, rotate_and_copy } from '$lib/rust_functions';
	import successIcon from '$lib/images/success.svg';
	import failedIcon from '$lib/images/error.svg';

	import '@fancyapps/ui/dist/fancybox/fancybox.css';
	import { rotate90, rotate180, rotate270 } from '$lib/rotate_front_end.js';
	import BiggerPicture from 'bigger-picture/svelte';
	import 'bigger-picture/css';

	//setup paths
	let main_dest_path = '';
	const unsub_mainDestPath = mainDestPath.subscribe((value) => (main_dest_path = value));
	let selected_school = '';
	const unsub_selectedSchool = selectedSchool.subscribe((value) => (selected_school = value));
	let image_list = [];
	const unsub_photo_path = photo_paths.subscribe((value) => (image_list = value));

	let curr_rot_deg = 0;

	onDestroy(() => {
		unsub_mainDestPath();
		unsub_selectedSchool();
		unsub_photo_path();
	});
	// initialize bigger picture
	let bp = BiggerPicture({
		target: document.body
	});
	onMount(async () => {
		const imageLinks = document.querySelectorAll('#images > a');
		// add click listener to open BiggerPicture
		for (let link of imageLinks) {
			link.addEventListener('click', openGallery);
		}
		// grab image links
		function openGallery(e) {
			e.preventDefault();
			//console.log('bp:', bp);
			get_higher_rotated_image(image_list[curr_index], curr_rot_deg).then((img_src) => {
				high_res_rotated_im_src = img_src;
				setTimeout(() => {
					bp.open({
						items: imageLinks,
						el: e.currentTarget
					});
				}, 0);
			});
		}
		// Prepare first photo to display after selecting a folder
		if (image_list.length > 0) {
			console.log('First execution!');
			getPhoto(image_list[curr_index]);
		}
	});

	//UNIMPLEMENTED caching mechanism
	const MAX_LAST_IMAGES = 10;
	let last_images = [];

	let high_res_rotated_im_src = '';
	let img_source = '';
	let img_path = '';
	const getPhoto = async (path) => {
		img_source = await invoke('get_photo', { path: path });
	};

	let curr_index = 0;
	$: img_path = image_list[curr_index];

	let curr_identity = {
		child_id: '113',
		roll_shot: ''
	};

	$: if (image_list[curr_index]) {
		let _path = image_list[curr_index];
		let _path_splitted = _path.split('\\');
		curr_identity.roll_shot = _path_splitted[_path_splitted.length - 1]
			.replace('DSC0', '')
			.replace('.JPG', '');
	}

	const getCurrentPhoto = async () => {
		getPhoto(image_list[curr_index]);
	};
	const go_next = async () => {
		if (curr_index < image_list.length) {
			curr_index++;
		}
		getCurrentPhoto();
	};
	const go_prev = async () => {
		if (curr_index > 0) {
			curr_index--;
		}
		getCurrentPhoto();
	};

	const set_curr_rot_deg = (new_deg) => {
		curr_rot_deg = new_deg;
	};
	let file_rotated_and_saved_show = false;
	let file_rotated_and_saved_success = false;
	const acceptImage = () => {
		console.log('img path:', img_path);
		let base_name = img_path.split('\\').pop();
		let dest_base_name = curr_identity.child_id + '_' + curr_identity.roll_shot + '.JPG';
		let dest_path = main_dest_path + selected_school + '\\' + dest_base_name;
		console.log('dest_path:', dest_path);
		//console.log('source basename:', base_name);
		//console.log('dest basename:', dest_base_name);
		setTimeout(() => {
			file_rotated_and_saved_show = true;
			file_rotated_and_saved_success = rotate_and_copy(curr_rot_deg, img_path, dest_path);
			setTimeout(() => {
				file_rotated_and_saved_show = false;
			}, 3000);
		}, 0);
	};
</script>

<div
	class="flex gap-y-3 h-[100%] min-h-full content-center justify-center border-dashed border-red-500"
>
	<div class="relative w-[50%] h-[100%] border flex flex-col ">
		<div id="images">
			<a
				href={high_res_rotated_im_src}
				data-img={high_res_rotated_im_src}
				data-thumb={img_source}
				data-alt="This is one option from photos you may select"
				data-height="800"
				data-width="600"
			>
				<img src={img_source} alt="This is one option from photos you may select" id="active" />
			</a>
		</div>

		<div class="absolute bottom-0 flex flex-col gap-y-2 border w-full">
			<div class="flex gap-x-3">
				<div class="flex flex-col gap-x-2">
					<label for="child_id" class="flex w-28 text-sm text-left">Child id</label>
					<input
						type="text"
						id="child_id"
						class="border focus:outline-none"
						bind:value={curr_identity.child_id}
					/>
				</div>
				<div class="flex flex-col gap-x-2">
					<label for="roll_shot" class="flex w-28 text-sm text-left">#Roll number</label>
					<input
						type="text"
						id="roll_shot"
						class="border focus:outline-none"
						bind:value={curr_identity.roll_shot}
						disabled
					/>
				</div>
				<div class="flex items-center self-end">
					{#if file_rotated_and_saved_show}
						{#if file_rotated_and_saved_success}
							<img src={successIcon} alt="success icon" />
							<p class="text-xs font-bold text-green-700">Photo succesfully copied</p>
						{:else}
							<img src={failedIcon} alt="failed icon" />
							<p class="text-xs font-bold text-red-600">Failed copying photo</p>
						{/if}
					{/if}
				</div>
			</div>
			<div class="flex gap-x-2 border w-full">
				<button class="border rounded-md px-4 py-2 bg-white text-gray-800" on:click={go_prev}
					>Prev</button
				>
				<button class="border rounded-md px-4 py-2 bg-white text-gray-800" on:click={go_next}
					>Next</button
				>
				<button
					class={`order rounded-md px-4 py-2 text-gray-800 ${
						curr_rot_deg === 90 ? 'bg-blue-400 text-white' : 'bg-white'
					}`}
					on:click={() => set_curr_rot_deg(rotate90('active'))}>rot 90</button
				>
				<button
					class={`order rounded-md px-4 py-2 text-gray-800 ${
						curr_rot_deg === 180 ? 'bg-blue-400 text-white' : 'bg-white'
					}`}
					on:click={() => set_curr_rot_deg(rotate180('active'))}>rot 180</button
				>
				<button
					class={`order rounded-md px-4 py-2 text-gray-800 ${
						curr_rot_deg === 270 ? 'bg-blue-400 text-white' : 'bg-white'
					}`}
					on:click={() => set_curr_rot_deg(rotate270('active'))}>rot 270</button
				>
				<button class="border rounded-md p-8 py-2 bg-green-600 text-gray-200" on:click={acceptImage}
					>Accept</button
				>
			</div>
		</div>
	</div>
</div>
