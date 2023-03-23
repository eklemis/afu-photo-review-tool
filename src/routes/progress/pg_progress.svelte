<script>
	//@ts-nocheck
	import {
		get_pg_stats_all,
		get_all_child_ids,
		get_child_ids_of,
		get_afu_of
	} from '$lib/rust_functions';
	import { onDestroy, onMount } from 'svelte';
	import Chart from 'chart.js/auto';
	import { mainDestPath } from '../../lib/store';
	import { createEventDispatcher } from 'svelte';
	const dispatch = createEventDispatcher();

	const dispatchRefreshed = () => {
		dispatch('refreshed', {});
	};

	//setup paths
	let main_dest_path = '';
	const unsub_mainDestPath = mainDestPath.subscribe((value) => (main_dest_path = value));
	onDestroy(unsub_mainDestPath);

	//PROPS
	export let pg_id = 0;
	export let refresh_now = false;

	//global variables
	let chart;
	let pg_schools = [];

	//table navigation control
	//1 for Incorect Ids, 2 for incomplete Ids
	let page_num = 1;

	//sub navigations in each pages
	//1 for option 'All' and keep increasing
	let page_one_sub = 1;
	//set selected school based on page_one_sub option
	let selected_school = 'NOT SET';
	//Child ids from pg_id in selected school
	let page_one_ids = {
		elig_ids: [],
		inelig_ids: []
	};
	//contain afu of unknown or misplaced photo
	let page_one_rows = {
		elig_rows: [],
		inelig_rows: []
	};
	const setSchool = async (new_school) => {
		selected_school = new_school;
		page_one_rows = {
			elig_rows: [],
			inelig_rows: []
		};
		page_one_ids = await get_child_ids_of(pg_id, selected_school);
		const collected_elig_ids = await get_all_child_ids(main_dest_path + selected_school);
		//console.log('collected ids:', collected_elig_ids);
		const colleted_inelig_ids = await get_all_child_ids(
			main_dest_path + 'Ineligibles\\' + selected_school
		);
		const filtered_eligs = collected_elig_ids.filter(
			(id) => !page_one_ids.elig_ids.includes(Number(id))
		);

		const fetchAfu = (ids) => {
			return new Promise((res) => {
				const all_afus = [];
				ids.forEach((id, idx) => {
					get_afu_of(id).then((afu) => {
						let formated_afu = { ...afu };
						if (afu.child_id === 0) {
							formated_afu.child_id = id;
						}
						all_afus.push(formated_afu);
						if (idx === ids.length - 1) {
							res(all_afus);
						}
					});
				});
			});
		};

		page_one_rows.elig_rows = await fetchAfu(filtered_eligs);

		const filter_ineligs = colleted_inelig_ids
			.filter((id) => !page_one_ids.inelig_ids.includes(id))
			.map(async (id) => {
				let afu = await get_afu_of(id);
				if (afu.child_id === 0) {
					afu.child_id = id;
				}
				return afu;
			});
		page_one_rows.inelig_rows = await fetchAfu(filter_ineligs);
		console.log('page_one_ids: ', page_one_ids);
	};
	const setPageOneSub = async (new_num) => {
		page_one_sub = new_num;
		let sch = pg_schools.filter((sch, idx) => idx === page_one_sub - 1)[0];
		await setSchool(sch);
	};

	//1 for option 'All', 2, 'Eligible', 3 for 'Ineligible'
	let page_two_sub = 1;
	//set selected school based on page_one_sub option
	let p2_selected_school = 'NOT SET';
	let latest_status = 'All';
	let not_collected_afu = {
		eligs: [],
		ineligs: []
	};

	const setPageTwoSub = async (new_num) => {
		page_two_sub = new_num;
		let sch = pg_schools.filter((sch, idx) => idx === page_two_sub - 1)[0];
		await setSchoolP2(sch);
	};
	const setSchoolP2 = async (new_school) => {
		not_collected_afu = {
			eligs: [],
			ineligs: []
		};
		p2_selected_school = new_school;
		//get all child ids of current photographer for current school
		const all_child_ids = await get_child_ids_of(pg_id, p2_selected_school);
		const collected_elig_ids = await get_all_child_ids(main_dest_path + p2_selected_school);
		const collected_inelig_ids = await get_all_child_ids(
			main_dest_path + 'Ineligiles\\' + p2_selected_school
		);
		//filter the collected ids to exclude incorrect ids
		const filtered_elig_ids = collected_elig_ids
			.filter((id) => all_child_ids.elig_ids.includes(Number(id)))
			.map((id) => Number(id));
		console.log('Filtered collected ids:', filtered_elig_ids);
		const filtered_inelig_ids = collected_inelig_ids
			.filter((id) => all_child_ids.inelig_ids.includes(Number(id)))
			.map((id) => Number(id));
		//separate the not collecteds
		const nc_elig_ids = all_child_ids.elig_ids.filter(
			(id) => !filtered_elig_ids.includes(Number(id))
		);
		const nc_inelig_ids = all_child_ids.inelig_ids.filter(
			(id) => !filtered_inelig_ids.includes(Number(id))
		);

		const fetchAfu = (ids) => {
			return new Promise((res) => {
				const all_afus = [];
				ids.forEach(async (id, idx) => {
					get_afu_of(id).then((afu) => {
						let formated_afu = { ...afu };
						if (afu.child_id === 0) {
							formated_afu.child_id = id;
						}
						all_afus.push(formated_afu);
						if (idx === ids.length - 1) {
							res(all_afus);
						}
					});
				});
			});
		};
		not_collected_afu.eligs = await fetchAfu(nc_elig_ids);
		not_collected_afu.ineligs = await fetchAfu(nc_inelig_ids);
	};
	//General Things
	const setSubsDefault = () => {
		page_one_sub = 1;
		setPageOneSub(1);
		page_two_sub = 1;
		setPageTwoSub(page_two_sub);
	};

	$: if (refresh_now) {
		setSubsDefault();
		get_pg_stats_all(pg_id).then(async (pg_stats) => {
			//let schoo_stat = pg_stats.map((stat) => stat.school);
			pg_stats.forEach((item) => {
				console.log(item);
			});
			const schools = pg_stats.map((stat) => stat.school);

			pg_schools = schools;

			const eligs = pg_stats.map((stat) => stat.num_elig);
			const ineligs = pg_stats.map((stat) => stat.num_inelig);

			const getNumCollectedEligs = async (schls) => {
				return new Promise((res) => {
					const collectedEligs = [];
					schls.forEach((sch, idx) => {
						get_all_child_ids(main_dest_path + sch).then((child_ids) => {
							//filter ids that not correspond to current sch
							get_child_ids_of(pg_id, sch).then((ids) => {
								console.log('ids: ', ids);
								const filtered_eligs = child_ids.filter((id) => ids.elig_ids.includes(Number(id)));
								collectedEligs.push(filtered_eligs.length);
								if (idx === schls.length - 1) {
									res(collectedEligs);
								}
							});
						});
					});
				});
			};
			const collectedEligs = await getNumCollectedEligs(schools);
			const getNumCollectedIneligs = async (schls) => {
				return new Promise((res) => {
					const collectedEligs = [];
					schls.forEach((sch, idx) => {
						get_all_child_ids(main_dest_path + 'Ineligibles\\' + sch).then((child_ids) => {
							collectedEligs.push(child_ids.length);
							if (idx === schls.length - 1) {
								res(collectedEligs);
							}
						});
					});
				});
			};
			const collectedIneligs = await getNumCollectedIneligs(schools);

			const notCollectedEligs = eligs.map((elig, idx) => {
				return elig - collectedEligs[idx];
			});

			const notCollectedIneligs = ineligs.map((elig, idx) => {
				return elig - collectedIneligs[idx];
			});

			if (chart) {
				chart.data.labels = schools;
				chart.data.datasets[0].data = collectedEligs;
				chart.data.datasets[1].data = notCollectedEligs;
				chart.data.datasets[2].data = collectedIneligs;
				chart.data.datasets[3].data = notCollectedIneligs;
				chart.update();

				dispatchRefreshed();
			}
		});
	}

	onMount(async () => {
		let ctx = document.getElementById('myChart');
		//CHART SETUPS
		const data = {
			labels: [], //schools,
			datasets: [
				{
					label: 'Collected Eligible',
					data: [], //collectedEligs,
					borderColor: 'rgb(75, 220, 152)',
					backgroundColor: 'rgba(75, 220, 152, 0.6)',
					borderWidth: 1,
					borderRadius: 1,
					borderSkipped: false,
					stack: 'Stack 0'
				},
				{
					label: 'NOT COLLECTED Eligible',
					data: [], //notCollectedElig,
					borderColor: 'rgb(255, 205, 86)',
					backgroundColor: 'rgba(255, 205, 86, 0.6)',
					borderWidth: 1,
					borderRadius: 1,
					borderSkipped: false,
					stack: 'Stack 0'
				},
				{
					label: 'Collected Ineligible',
					data: [], //collectedIneligs,
					borderColor: 'rgb(75, 202, 232)',
					backgroundColor: 'rgba(75, 202, 255, 0.6)',
					borderWidth: 1,
					borderRadius: 1,
					borderSkipped: false,
					stack: 'Stack 1'
				},
				{
					label: 'NOT COLLECTED Ineligible',
					data: [], //notCollectedInelig,
					borderColor: 'rgb(230, 225, 102)',
					backgroundColor: 'rgba(230, 225, 102, 0.6)',
					borderWidth: 1,
					borderRadius: 1,
					borderSkipped: false,
					stack: 'Stack 1'
				}
			]
		};
		const config = {
			type: 'bar',
			data: data,
			options: {
				plugins: {
					title: {
						display: true,
						text: 'Photo Collecting Progress'
					}
				},
				responsive: true,
				interaction: {
					intersect: false
				},
				scales: {
					x: {
						stacked: true
					},
					y: {
						stacked: true
					}
				}
			}
		};

		chart = new Chart(ctx, { ...config });
	});
	//});
</script>

<div class="flex flex-col w-full">
	<div class="border w-full p-1">
		<canvas id="myChart" />
	</div>

	<div class="border w-full p-2">
		<div class="flex items-center justify-between border p-1">
			<h3 class="text-[12px] text-gray-700 font-bold">
				{page_num === 1 ? 'Incorrect Name or Misplaced Photos List' : 'Photo Not Collected List'}
			</h3>
			<nav class="flex items-center justify-between">
				<button
					class="border p-1 text-sm text-slate-600 disabled:text-slate-300 w-6 h-6 flex items-center justify-center"
					on:click={() => (page_num = 1)}
					disabled={page_num == 1}>{'<'}</button
				>
				<button
					class="border p-1 text-sm text-slate-600 disabled:text-slate-300 w-6 h-6 flex items-center justify-center"
					on:click={() => (page_num = 2)}
					disabled={page_num == 2}>{'>'}</button
				>
			</nav>
		</div>
		<div class="p-1 border border-t-0">
			{#if page_num === 1}
				<ul class="flex gap-[2px] mt-2 mb-2 flex-wrap">
					{#each pg_schools as pg_sch, idx ('bt-' + pg_sch)}
						<li class="w-54">
							<button
								on:click={() => setPageOneSub(idx + 1)}
								class={`text-[12px] w-full border border-slate-200 p-1 ${
									page_one_sub === idx + 1
										? 'text-gray-600 bg-slate-300'
										: 'text-gray-600 border-gray-400'
								}`}>{pg_sch}</button
							>
						</li>
					{/each}
				</ul>

				<table class="border-collapse border border-slate-400">
					<thead>
						<th
							class="border border-slate-300 text-[12px] text-gray-500 font-normal bg-slate-100 p-1"
							>No.</th
						>
						<th
							class="border border-slate-300 text-[12px] text-gray-500 font-normal bg-slate-100 p-1"
							>Child id</th
						>
						<th
							class="border border-slate-300 text-[12px] text-gray-500 font-normal bg-slate-100 p-1"
							>Correct Identity</th
						>
					</thead>
					<tbody class="max-h-96 overflow-scroll">
						{#if page_one_rows.elig_rows.length + page_one_rows.inelig_rows.length > 0}
							{#each page_one_rows.elig_rows as afu, idx ('page_one_row-' + idx)}
								<tr>
									<td class="border border-slate-300 text-gray-700 text-[12px] p-1">{idx + 1}</td>
									<td class="border border-slate-300 text-gray-700 text-[12px] p-1"
										>{afu.child_id}</td
									>

									<td class="border border-slate-300 text-gray-700 text-[12px] p-1"
										>{afu.child_name +
											', ' +
											afu.school +
											', grade: ' +
											afu.last_grade +
											', ' +
											afu.last_status}</td
									>
								</tr>
							{/each}
						{:else}
							<p class="flex items-center justify-center w-full text-gray-500">No Records</p>
						{/if}
					</tbody>
				</table>
			{:else}
				<ul class="flex gap-[2px] mt-2 mb-2 flex-wrap">
					{#each pg_schools as pg_sch, idx ('bt2-' + pg_sch)}
						<li class="w-54">
							<button
								on:click={() => setPageTwoSub(idx + 1)}
								class={`text-[12px] w-full border border-slate-200 p-1 ${
									page_two_sub === idx + 1
										? 'text-gray-600 bg-slate-300'
										: 'text-gray-600 border-gray-400'
								}`}>{pg_sch}</button
							>
						</li>
					{/each}
				</ul>

				<table class="border-collapse border border-slate-400">
					<thead>
						<th
							class="border border-slate-300 text-[12px] text-gray-500 font-normal bg-slate-100 p-1"
							>No.</th
						>
						<th
							class="border border-slate-300 text-[12px] text-gray-500 font-normal bg-slate-100 p-1"
							>Child id</th
						>
						<th
							class="border border-slate-300 text-[12px] text-gray-500 font-normal bg-slate-100 p-1"
							>Name</th
						>
						<th
							class="border border-slate-300 text-[12px] text-gray-500 font-normal bg-slate-100 p-1"
							>School</th
						>
						<th
							class="border border-slate-300 text-[12px] text-gray-500 font-normal bg-slate-100 p-1"
							>Latest Status</th
						>
					</thead>
					<tbody>
						{#if not_collected_afu.eligs.length + not_collected_afu.ineligs.length > 0}
							{#each not_collected_afu.eligs as afu, idx ('nc-elig' + idx)}
								<tr>
									<td class="border border-slate-300 text-gray-700 text-[12px] p-1">{idx + 1}</td>
									<td class="border border-slate-300 text-gray-700 text-[12px] p-1"
										>{afu.child_id}</td
									>
									<td class="border border-slate-300 text-gray-700 text-[12px] p-1"
										>{afu.child_name}</td
									>
									<td class="border border-slate-300 text-gray-700 text-[12px] p-1">{afu.school}</td
									>
									<td class="border border-slate-300 text-gray-700 text-[12px] p-1"
										>{afu.last_status}</td
									>
								</tr>
							{/each}
							{#each not_collected_afu.ineligs as afu, idx ('nc-inelig' + idx)}
								<tr>
									<td class="border border-slate-300 text-gray-700 text-[12px] p-1"
										>{idx + not_collected_afu.eligs.length + 1}</td
									>
									<td class="border border-slate-300 text-gray-700 text-[12px] p-1"
										>{afu.child_id}</td
									>
									<td class="border border-slate-300 text-gray-700 text-[12px] p-1"
										>{afu.child_name}</td
									>
									<td class="border border-slate-300 text-gray-700 text-[12px] p-1">{afu.school}</td
									>
									<td class="border border-slate-300 text-gray-700 text-[12px] p-1"
										>{afu.last_status}</td
									>
								</tr>
							{/each}
						{:else}
							<p class="flex items-center justify-center w-full text-gray-500">No Records</p>
						{/if}
					</tbody>
				</table>
			{/if}
		</div>
	</div>
</div>
