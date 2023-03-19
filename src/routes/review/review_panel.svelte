<script>
	// @ts-nocheck
	import { onMount, onDestroy } from 'svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	import { photo_paths, mainDestPath, selectedSchool } from '../../lib/store';
	import {
		get_higher_rotated_image,
		rotate_and_copy,
		get_all_child_ids,
		create_nonexist_folders,
		get_afu_of,
		get_all_photographers,
		get_photographer_of
	} from '$lib/rust_functions';
	import successIcon from '$lib/images/success.svg';
	import failedIcon from '$lib/images/error.svg';

	import '@fancyapps/ui/dist/fancybox/fancybox.css';
	import BiggerPicture from 'bigger-picture/svelte';
	import 'bigger-picture/css';
	import ProgressPanel from './progress_panel.svelte';

	//setup paths
	let main_dest_path = '';
	const unsub_mainDestPath = mainDestPath.subscribe((value) => (main_dest_path = value));
	let selected_school = '';
	const unsub_selectedSchool = selectedSchool.subscribe((value) => (selected_school = value));
	let image_list = [];
	const unsub_photo_path = photo_paths.subscribe((value) => (image_list = value));

	let curr_rot_deg = 0;
	let contain_identity = false;
	let ids_with_photos = [];
	let ids_with_photos_inel = [];

	const set_new_identity = (new_idt) => {
		preserve_identity_image = Object.assign({}, identity_image);
		identity_image = { ...new_idt };
	};

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
		const idtLinks = document.querySelectorAll('#idt-image');

		// add click listener to open BiggerPicture
		for (let link of imageLinks) {
			link.addEventListener('click', openGallery);
		}
		// add click listener to open BiggerPicture
		for (let link of idtLinks) {
			link.addEventListener('click', openGalleryDt);
		}
		// grab image links
		function openGallery(e) {
			e.preventDefault();
			//console.log('bp:', bp);
			if (
				image_list[curr_index] !== high_res_rotated_im.path ||
				high_res_rotated_im.deg !== curr_rot_deg
			) {
				console.log('Get higher res image');
				get_higher_rotated_image(image_list[curr_index], curr_rot_deg).then((img_src) => {
					high_res_rotated_im.src = img_src;
					high_res_rotated_im.path = image_list[curr_index];
					high_res_rotated_im.deg = curr_rot_deg;
					setTimeout(() => {
						bp.open({
							items: imageLinks,
							el: e.currentTarget
						});
					}, 0);
				});
			} else {
				setTimeout(() => {
					bp.open({
						items: imageLinks,
						el: e.currentTarget
					});
				}, 0);
			}
		} // grab image links
		function openGalleryDt(e) {
			e.preventDefault();
			if (
				identity_image.path !== preserve_identity_image.path ||
				identity_image.rot !== preserve_identity_image.rot
			) {
				console.log('Getting higher res identity image');
				get_higher_rotated_image(identity_image.path, identity_image.rot).then((img_src) => {
					identity_image.src = img_src;
					preserve_identity_image = Object.assign({}, identity_image);
					console.log('got new identity image:', preserve_identity_image);
					setTimeout(() => {
						bp.open({
							items: idtLinks,
							el: e.currentTarget
						});
					}, 0);
				});
			} else {
				setTimeout(() => {
					bp.open({
						items: idtLinks,
						el: e.currentTarget
					});
				}, 0);
			}
		}
		// Prepare first photo to display after selecting a folder
		if (image_list.length > 0) {
			getPhoto(image_list[curr_index]);
		}
		get_all_child_ids(main_dest_path + selected_school).then((ids) => (ids_with_photos = ids));
		get_all_child_ids(main_dest_path + 'Ineligibles\\' + selected_school).then(
			(ids) => (ids_with_photos_inel = ids)
		);
	});

	//UNIMPLEMENTED caching mechanism
	const MAX_LAST_IMAGES = 10;
	let last_images = [];

	let rejected_folder_checked = false;
	let fresh_start = true;

	let high_res_rotated_im = { src: '', path: '', deg: 0 };
	let img_source = '';
	let img_path = '';
	let ratio_base = 300;
	let ratio_three = ratio_base * 3;
	let ratio_four = ratio_base * 4;
	let img_width = ratio_three;
	let img_height = ratio_four;
	$: if (curr_rot_deg === 0) {
		img_width = ratio_four;
		img_height = ratio_three;
	} else {
		img_width = ratio_three;
		img_height = ratio_four;
	}
	const getPhoto = async (path) => {
		img_source = await invoke('get_rotated_image_tumb', { srcPath: path, deg: curr_rot_deg });
	};

	let identity_image = {
		path: '',
		src: '',
		rot: 0,
		width: img_width,
		height: img_height
	};
	let preserve_identity_image = {
		path: '',
		src: '',
		rot: 0,
		width: img_width,
		height: img_height
	};

	let curr_index = 0;
	$: img_path = image_list[curr_index];

	let id_input;
	let curr_identity = {
		child_id: '113',
		roll_shot: ''
	};
	let curr_afu = {
		child_id: 0,
		child_name: '',
		sex: '',
		last_grade: '',
		last_status: '',
		school: '',
		community: '',
		pg_id: 0,
		smile_score: 0,
		bg_score: 0,
		clarity_score: 0
	};
	let curr_pg = {
		id: 0,
		name: 0
	};
	const refreshCollectedList = async () => {
		if (curr_afu.school && curr_afu.school.trim().length > 0) {
			selected_school = curr_afu.school;
		}
		await get_all_child_ids(main_dest_path + selected_school).then(
			(ids) => (ids_with_photos = ids)
		);
		await get_all_child_ids(main_dest_path + 'Ineligibles\\' + selected_school).then(
			(ids) => (ids_with_photos_inel = ids)
		);
		console.log('Collected Elig Of ', selected_school, ids_with_photos);
		console.log('Collected INElig Of ', selected_school, ids_with_photos_inel);
	};
	$: if (curr_afu.pg_id === 0 && curr_pg.id === 0) {
		get_photographer_of(selected_school).then((pg) => {
			curr_pg = pg;
			console.log(`Initialize PG ${selected_school}: ${curr_pg.id}, ${curr_pg.name}`);
			refreshCollectedList();
			refresh_chart = true;
		});
	} else if (curr_afu.pg_id !== 0 && curr_afu.pg_id !== curr_pg.id) {
		get_all_photographers().then((pgs) => {
			curr_pg = pgs.filter((pg) => pg.id === curr_afu.pg_id)[0];
			console.log('PG Change By Child Id:', curr_pg.id, curr_pg.name);
			refreshCollectedList();
			refresh_chart = true;
		});
	}
	/*$: console.log('curr_pg:', curr_pg);*/
	const identity_tick = () => {
		if (contain_identity) {
			curr_identity.child_id = '113';
			//reset identity photo
			set_new_identity({
				path: image_list[curr_index],
				src: img_source,
				rot: curr_rot_deg,
				width: img_width,
				height: img_height
			});
			id_input.focus();
			fresh_start = false;
		}
	};
	$: if (image_list[curr_index]) {
		let _path = image_list[curr_index];
		let _path_splitted = _path.split('\\');
		let _last_part = _path_splitted[_path_splitted.length - 1].replace('.JPG', '');
		let _roll_shot = _last_part;
		if (_last_part.length > 3) {
			_roll_shot = _last_part.substring(_last_part.length - 4);
		}
		curr_identity.roll_shot = _roll_shot;
	}

	let is_child_id = false;
	$: is_child_id =
		/^1130\d{4}$/.test(curr_identity.child_id) || /^1131\d{4}$/.test(curr_identity.child_id);

	const getCurrentPhoto = async () => {
		contain_identity = false;
		getPhoto(image_list[curr_index]);
	};
	const go_next = async () => {
		if (curr_index + 1 < image_list.length) {
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

	const reset_identity_images = () => {
		identity_image.src = img_source;
		identity_image.deg = curr_rot_deg;
		preserve_identity_image.src = img_source;
		preserve_identity_image.deg = curr_rot_deg;
		identity_image.width = img_width;
		identity_image.height = img_height;
		preserve_identity_image.width = img_width;
		preserve_identity_image.height = img_height;
	};
	const set_curr_rot_deg = (new_deg) => {
		if (new_deg !== curr_rot_deg) {
			curr_rot_deg = new_deg;
			getCurrentPhoto();
			if (contain_identity) {
				get_higher_rotated_image(identity_image.path, curr_rot_deg).then((img_src) => {
					reset_identity_images();
				});
			}
		}
		// this request is to tuned-off the rotation
		// BUG HERE
		else {
			curr_rot_deg = 0;
			get_higher_rotated_image(img_path, curr_rot_deg).then((img_src) => {
				img_source = img_src;
				reset_identity_images();
			});
		}
		getCurrentPhoto();
	};
	let file_rotated_and_saved_show = false;
	let file_rotated_and_saved_success = false;
	let refresh_chart = true;
	const turnOffRefresh = () => {
		refresh_chart = false;
		console.log('Refresh Turned OFF!');
	};
	$: if (file_rotated_and_saved_success) {
		refresh_chart = true;
	}

	const acceptImage = () => {
		//let base_name = img_path.split('\\').pop();
		selected_school = curr_afu.school;
		let dest_base_name = curr_identity.child_id + '_' + curr_identity.roll_shot + '.JPG';
		let dest_path = main_dest_path + selected_school + '\\' + dest_base_name;
		console.log('dest_path:', dest_path);
		get_all_child_ids(main_dest_path + selected_school).then((ids) => (ids_with_photos = ids));
		setTimeout(() => {
			file_rotated_and_saved_show = true;
			file_rotated_and_saved_success = rotate_and_copy(curr_rot_deg, img_path, dest_path);
			setTimeout(() => {
				file_rotated_and_saved_show = false;
				refreshCollectedList();
			}, 3500);
		}, 0);
	};
	const moveWithIdentity = async () => {};
	const rejectImage = async () => {
		//Destination path for selected photo
		selected_school = curr_afu.school;
		let dest_base_name = curr_identity.child_id + '_' + curr_identity.roll_shot + '.JPG';
		let base_path_only = main_dest_path + 'Rejecteds\\' + selected_school;
		let dest_path = base_path_only + '\\' + dest_base_name;
		//Destination path for identity photo
		let dest_base_name_idt = curr_identity.child_id + '_IDENTITY.JPG';
		let dest_path_idt = base_path_only + '\\' + dest_base_name_idt;
		//Create necessary folder if not exists
		if (!rejected_folder_checked) {
			create_nonexist_folders(base_path_only).then((res) => {
				rotate_and_copy(curr_rot_deg, img_path, dest_path).then((success_cp) => {
					if (success_cp) {
						rotate_and_copy(identity_image.rot, identity_image.path, dest_path_idt).then(
							(all_success) => {
								file_rotated_and_saved_show = all_success;
								setTimeout(() => {
									file_rotated_and_saved_show = false;
								}, 3500);
							}
						);
					}
				});
				rejected_folder_checked = true;
			});
		} else {
			rotate_and_copy(curr_rot_deg, img_path, dest_path).then((success_cp) => {
				if (success_cp) {
					rotate_and_copy(identity_image.rot, identity_image.path, dest_path_idt).then(
						(all_success) => {
							file_rotated_and_saved_show = all_success;
							setTimeout(() => {
								file_rotated_and_saved_show = false;
							}, 3500);
						}
					);
				}
			});
		}
	};
	const moveToIneligible = () => {};
</script>

<div class="flex flex-col h-[100%] min-h-full content-center justify-center border-dashed">
	<div class="relative w-full h-full flex">
		<div class="flex flex-col p-4 gap-y-4 w-[550px] border">
			<h3 class="text-sm font-bold text-[#423C3C]">Identity</h3>
			<div class="flex gap-x-2 items-baseline justify-between">
				<p class="flex w-28 text-sm text-gray-400 text-left">Child id</p>
				<span class="focus:outline-none w-16 bg-white border-b text-[#423C3C] text-sm"
					>{curr_afu.child_id}</span
				>
			</div>
			<div class="flex gap-x-2 items-baseline justify-between">
				<label for="roll_shot" class="flex w-28 text-sm text-gray-400 text-left">#Roll number</label
				>
				<input
					type="text"
					id="roll_shot"
					class="focus:outline-none w-16 bg-white border-b text-[#423C3C] text-sm text-right"
					bind:value={curr_identity.roll_shot}
					disabled
				/>
			</div>
			<div class="flex gap-x-2 items-baseline justify-between">
				<p class="flex w-28 text-sm text-gray-400 text-left">First name</p>
				<span class="text-[#423C3C] font-bold text-sm">{curr_afu.child_name.split(' ')[0]}</span>
			</div>
			<div class="flex gap-x-2 items-baseline justify-between">
				<p class="flex text-sm text-gray-400 text-left">School</p>
				<span class="text-[#423C3C] text-sm">{curr_afu.school}</span>
			</div>
			<div class="flex gap-x-2 items-baseline justify-between">
				<p class="flex w-28 text-sm text-gray-400 text-left">Latest status</p>
				{#if curr_afu.last_status === 'Eligible'}
					<span class={'font-bold italic text-sm text-green-600'}>{curr_afu.last_status}</span>
				{:else}
					<span class={'font-bold italic underline text-sm text-yellow-500'}
						>{curr_afu.last_status}</span
					>
				{/if}
			</div>
			<a
				id="idt-image"
				href={preserve_identity_image.src}
				data-img={preserve_identity_image.src}
				data-thumb={img_source}
				data-alt="will open photo with identity"
				data-height={preserve_identity_image.height}
				data-width={preserve_identity_image.width}
				class="mt-16 h-9 bg-[#FAFAFA] border border-[#405CF5] rounded text-[#405CF5] text-center font-semibold text-sm flex items-center justify-center"
				>See identity photo</a
			>
			<div class="flex gap-x-3 p-1 mt-12">
				<div class="flex items-center self-end">
					{#if ids_with_photos.includes(curr_identity.child_id)}
						<p class="text-xs font-bold text-yellow-400">Child already has photo</p>
					{/if}
					{#if file_rotated_and_saved_show}
						{#if file_rotated_and_saved_success}
							<img src={successIcon} class="w-6 h-6" alt="success icon" />
							<p class="text-xs font-bold text-green-700">Photo succesfully copied</p>
						{:else}
							<img src={failedIcon} alt="failed icon" class="w-6 h-6" />
							<p class="text-xs font-bold text-red-600">Failed copying photo</p>
						{/if}
					{/if}
				</div>
			</div>
		</div>
		<div class="flex flex-col w-full">
			<div class="flex items-center justify-end px-2 w-full bg-base-blue">
				<div class="flex items-center gap-x-1 py-1">
					<button
						class={`order h-7 w-14 text-xs font-bold rounded-sm px-4 py-2 flex items-center justify-center ${
							curr_rot_deg === 90 ? ' text-white bg-blue-400 ' : 'bg-[#F8F9FA] text-font-blue'
						}`}
						on:click={() => set_curr_rot_deg(90)}>90°</button
					>
					<button
						class={`order h-7 w-14 text-xs font-bold rounded-sm px-4 py-2 flex items-center justify-center ${
							curr_rot_deg === 180 ? ' text-white bg-blue-400 ' : 'bg-[#F8F9FA] text-font-blue'
						}`}
						on:click={() => set_curr_rot_deg(180)}>180°</button
					>
					<button
						class={`order h-7 w-14 text-xs font-bold rounded-sm px-4 py-2 flex items-center justify-center ${
							curr_rot_deg === 270 ? ' text-white bg-blue-400 ' : 'bg-[#F8F9FA] text-font-blue'
						}`}
						on:click={() => set_curr_rot_deg(270)}>270°</button
					>
				</div>
			</div>
			<div
				class="flex flex-col items-center justify-center bg-[#FAFAFA] border w-full min-h-[600px] h-4/5"
			>
				<div class="flex w-full items-center justify-center">
					<div class="flex items-center text-sm gap-x-1">
						<span class="font-bold text-[#423C3C]">{curr_index + 1}</span>
						<span class="text-[#423C3C]">of {image_list.length}</span>
					</div>
				</div>
				<div id="images" class="flex items-center justify-center min-h-[520px] border">
					<a
						href={high_res_rotated_im.src}
						data-img={high_res_rotated_im.src}
						data-thumb={img_source}
						data-height={img_height}
						data-width={img_width}
						data-alt="This is one option from photos you may select"
					>
						<img
							class="rounded-sm"
							src={img_source}
							alt="This is one option from photos you may select"
							id="active"
							width="390"
							height="520"
						/>
					</a>
				</div>
				<div class="relative flex w-[392px] items-center justify-between px-1 pt-1">
					<div
						class={`absolute items-center justify-center w-full bg-[#EB6262] h-12 left-0 -top-12 rounded-b 
						${contain_identity ? 'flex' : 'hidden'}`}
					>
						<label for="child_id" class="flex w-20 text-sm text-white text-left">Child id</label>
						<input
							type="text"
							id="child_id"
							bind:this={id_input}
							class="focus:outline-none w-20 rounded bg-white border border-[F0F0F0] text-[#423C3C] text-sm h-8"
							bind:value={curr_identity.child_id}
							on:keyup={async () => {
								if (is_child_id) {
									curr_afu = await get_afu_of(curr_identity.child_id);
								}
							}}
							disabled={!contain_identity}
						/>
					</div>
					<div>
						<button
							class="border border-[#405CF5] rounded text-sm px-2 w-12 h-7 bg-white text-[#405CF5]"
							on:click={go_prev}>Prev</button
						>
						<button
							class="border border-[#405CF5] rounded text-sm px-2 w-12 h-7 bg-white text-[#405CF5]"
							on:click={go_next}
							disabled={!is_child_id && !fresh_start}>Next</button
						>
					</div>
					<div class="flex items-center gap-x-1">
						<label for="contain_identity" class="text-sm text-[#6A6C77]"
							>current photo contains new identity</label
						>
						<input
							id="contain_identity"
							type="checkbox"
							bind:checked={contain_identity}
							on:change={identity_tick}
						/>
					</div>
				</div>
			</div>
			<div class="flex h-16 w-full p-1 gap-x-2 justify-between">
				<div>
					<button
						class="border h-10 w-20 rounded-md bg-green-600 text-white font-bold text-sm"
						on:click={acceptImage}
						disabled={contain_identity}>Accept</button
					>
					<button
						class="border h-10 w-20 rounded-md bg-slate-100 text-gray-600 font-bold text-sm"
						on:click={() => {}}
						disabled={contain_identity}>Ineligible</button
					>
				</div>
				<button
					class="border h-10 w-20 rounded-md bg-[#EA4C4C] text-white font-bold text-sm"
					on:click={rejectImage}
					disabled={contain_identity}>Reject</button
				>
			</div>
		</div>
		<div class="h-full bg-[#FAFAFA] w-[500px] border ml-1">
			<div>
				<button class="text-xs">Photos in folder</button>
				<button class="text-xs">Progress</button>
			</div>
			<div>
				<ProgressPanel
					photographer={curr_pg}
					school={curr_afu.school === '' ? selected_school : curr_afu.school}
					collected_elig_ids={ids_with_photos}
					collected_inelig_ids={ids_with_photos_inel}
					should_refresh={refresh_chart}
					on:refreshed={turnOffRefresh}
				/>
			</div>
		</div>
	</div>
</div>
