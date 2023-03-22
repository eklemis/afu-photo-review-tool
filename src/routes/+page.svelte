<script>
	// @ts-nocheck
	import { photo_paths, mainDestPath, selectedSchool } from '../lib/store';
	import { invoke } from '@tauri-apps/api/tauri';
	import { create_nonexist_folders, get_all_schools } from '$lib/rust_functions';
	import { onDestroy, onMount } from 'svelte';
	let path = '';
	let is_path_exist = false;
	let school = '';

	//setup paths
	let main_dest_path = '';
	const unsub_mainDestPath = mainDestPath.subscribe((value) => (main_dest_path = value));
	/**
	 * @type {string[]}
	 */
	let fileList = [];
	let schools = [];
	const check_path = async () => {
		is_path_exist = await invoke('is_path_exist', { path });
		if (is_path_exist) {
			fileList = await invoke('get_file_list', { folderPath: path });
			photo_paths.set(fileList);
		}
	};
	onMount(async () => {
		await get_all_schools().then((all_schools) => {
			schools = all_schools;
		});
	});
	onDestroy(unsub_mainDestPath);
</script>

<div class="p-5 flex flex-col items-end content-center gap-y-4 w-full">
	<div class="w-full">
		<label for="photo folder" class="block text-sm font-medium leading-6 text-gray-900"
			>Source folder</label
		>
		<input
			type="text"
			id="photo folder"
			class="block w-full rounded-md border-0 h-9 px-4 text-gray-900 ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:outline-none focus:ring-indigo-600 sm:text-sm sm:leading-6"
			bind:value={path}
			on:change={() => check_path()}
		/>
	</div>
	<div class="w-full">
		<label for="school" class="block text-sm font-medium leading-6 text-gray-900"
			>Enter school name</label
		>
		<select
			bind:value={school}
			id="school"
			class="block w-full h-9 rounded-md border-0 py-1.5 px-4 text-gray-900 ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:outline-none focus:ring-indigo-600 sm:text-sm sm:leading-6"
		>
			{#each schools as school (school)}
				<option value={school}>{school}</option>
			{/each}
		</select>
	</div>
	<a
		href="/review"
		on:click={() => {
			selectedSchool.set(school);
			const _path = `${main_dest_path}${school}`;
			create_nonexist_folders(_path);
		}}
		class={`bg-yellow-600 w-48 self-start rounded-md p-2 text-white text-center ${
			is_path_exist ? '' : 'pointer-events-none'
		}`}>Start Review</a
	>
</div>
