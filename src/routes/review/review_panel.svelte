<script>
	// @ts-nocheck
	import { invoke } from '@tauri-apps/api/tauri';
	import { photo_paths } from '../../lib/store';

	const MAX_LAST_IMAGES = 10;
	let last_images = [];

	let img_source = '';
	const getPhoto = async (path) => {
		img_source = await invoke('get_photo', { path: path });
	};
	const getOcrInfo = async (src_path, deg) => {
		let ocr_info = await invoke('get_ocr_info', { srcPath: src_path, deg: deg });
		console.log('Ocr Info:', ocr_info);
		return ocr_info;
	};
	let image_list = [];
	photo_paths.subscribe((value) => (image_list = value));

	let curr_index = 0;
	let identity_image = {
		path: '',
		source: ''
	};
	let curr_identity = {
		child_id: '113',
		roll_shot: ''
	};
	let curr_rot_deg = 0;
	$: if (curr_index === 0 && image_list.length > 0) {
		getPhoto(image_list[curr_index]);
	}
	$: if (image_list[curr_index]) {
		let _path = image_list[curr_index];
		let _path_splitted = _path.split('\\');
		curr_identity.roll_shot = _path_splitted[_path_splitted.length - 1]
			.replace('DSC0', '')
			.replace('.JPG', '');
	}

	const getCurrentPhoto = async () => {
		getPhoto(image_list[curr_index]);
		console.log('current image:', image_list[curr_index]);
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
	const set_identity_image = async () => {
		identity_image.path = image_list[curr_index];
		identity_image.source = img_source;
		setTimeout(() => go_next(), 0);
	};
	const set_curr_rot_deg = (new_deg) => {
		curr_rot_deg = new_deg;
	};
	const rotate = (elId, deg) => {
		const rotated = document.getElementById(elId);
		rotated.style.transform = `rotate(${deg}deg)`;
		set_curr_rot_deg(deg);
	};
	const rotate90 = (elId) => {
		rotate(elId, 90);
	};
	const rotate180 = (elId) => {
		rotate(elId, 180);
	};
	const rotate270 = async (elId) => {
		if (elId === 'active') {
			rotate(elId, 270);
		} else {
			identity_image.source = await invoke('get_rotated_image', {
				srcPath: identity_image.path,
				deg: 270
			});
		}
	};
</script>

<div class="flex gap-y-3 h-[100%] min-h-full">
	<div class="relative w-[50%] min-h-full border flex-col">
		<img src={identity_image.source} alt="" id="identity" />
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
				on:click={() => rotate270('identity')}>rot 270</button
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
	<div class="relative w-[50%] h-[100%] border flex-col">
		<img src={img_source} alt="" id="active" />
		<div class="absolute bottom-0 flex gap-x-2 border">
			<button
				class="border rounded-md px-4 py-2 bg-white text-gray-800"
				on:click={set_identity_image}>Mark as identity</button
			>
			<button class="border rounded-md px-4 py-2 bg-white text-gray-800" on:click={go_prev}
				>Prev</button
			>
			<button class="border rounded-md px-4 py-2 bg-white text-gray-800" on:click={go_next}
				>Next</button
			>
			<button
				class="border rounded-md px-4 py-2 bg-white text-gray-800"
				on:click={() => rotate90('active')}>rot 90</button
			>
			<button
				class="border rounded-md p-8 py-2 bg-white text-gray-800"
				on:click={() => rotate180('active')}>rot 180</button
			>
			<button
				class="border rounded-md p-8 py-2 bg-white text-gray-800"
				on:click={() => rotate270('active')}>rot 270</button
			>
			<button class="border rounded-md p-8 py-2 bg-green-600 text-gray-200">Accept</button>
		</div>
	</div>
</div>
