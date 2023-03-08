<script>
	// @ts-nocheck
	import { photo_paths } from '../lib/store';

	import { invoke } from '@tauri-apps/api/tauri';
	const photographers = [
		'Arini',
		'Arias',
		'Putra',
		'Bhaskara',
		'Dinarti',
		'Yogi',
		'Oskarina',
		'Jevin',
		'Jemi',
		'Tomas',
		'Yemima',
		'Simon',
		'Desty',
		'Jibrail',
		'Bryan L.',
		'Anjelina'
	];
	let path = '';
	let is_path_exist = false;

	/**
	 * @type {string[]}
	 */
	let fileList = [];
	const check_path = async () => {
		console.log('path:', path);
		is_path_exist = await invoke('is_path_exist', { path });
		fileList = await invoke('get_file_list', { folderPath: path });
		photo_paths.set(fileList);
	};
</script>

<div class="p-5 flex flex-col gap-y-4 h-screen w-full">
	<div>
		<label for="photo folder" class="block text-sm font-medium leading-6 text-gray-900"
			>Folder path</label
		>
		<input
			type="text"
			id="photo folder"
			class="block w-full rounded-md border-0 py-1.5 pl-7 pr-20 text-gray-900 ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:outline-none focus:ring-indigo-600 sm:text-sm sm:leading-6"
			bind:value={path}
			on:change={() => check_path()}
		/>
	</div>
	<div>
		<label for="photographers" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white"
			>Select photographer</label
		>
		<select
			id="photographers"
			name="photographers"
			class="focus:outline-none px-4 bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
		>
			{#each photographers as pg ('op-' + pg)}
				<option value={pg}>{pg}</option>
			{/each}
		</select>
	</div>

	<a
		href="/review"
		class={`bg-red-500 rounded-md p-2 text-white text-center ${
			is_path_exist ? '' : 'pointer-events-none'
		}`}>Start Review</a
	>
</div>
