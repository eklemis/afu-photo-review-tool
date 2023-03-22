<script>
	//@ts-nocheck
	import { get_pg_stats_all, get_all_child_ids } from '$lib/rust_functions';
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
	let selected_school = 'ALL';
	const setPageOneSub = (new_num) => {
		page_one_sub = new_num;
		selected_school = pg_schools.filter((sch, idx) => idx === page_one_sub - 2)[0];
	};
	//1 for option 'All', 2, 'Eligible', 3 for 'Ineligible'
	let page_two_sub = 1;
	let latest_status = 'All';
	const setPageTwoSub = (new_num) => {
		page_two_sub = new_num;
		if (page_two_sub === 2) {
			latest_status = 'Eligible';
		} else if (page_two_sub === 3) {
			latest_status = 'Ineligible';
		} else {
			latest_status = 'All';
		}
	};
	const setSubsDefault = () => {
		page_one_sub = 1;
		selected_school = 'ALL';
		page_two_sub = 1;
		latest_status = 'All';
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
							collectedEligs.push(child_ids.length);
							if (idx === schls.length - 1) {
								res(collectedEligs);
							}
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
					<li class="w-54">
						<button
							on:click={() => setPageOneSub(1)}
							class={`text-[12px] w-full border border-slate-200 p-1 ${
								page_one_sub === 1 ? 'text-gray-600 bg-slate-300' : 'text-gray-600 border-gray-400'
							}`}>All school</button
						>
					</li>
					{#each pg_schools as pg_sch, idx ('bt-' + pg_sch)}
						<li class="w-54">
							<button
								on:click={() => setPageOneSub(idx + 2)}
								class={`text-[12px] w-full border border-slate-200 p-1 ${
									page_one_sub === idx + 2
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
							>Child id</th
						>
						<th
							class="border border-slate-300 text-[12px] text-gray-500 font-normal bg-slate-100 p-1"
							>#Shoot</th
						>
						<th
							class="border border-slate-300 text-[12px] text-gray-500 font-normal bg-slate-100 p-1"
							>Correct Identity</th
						>
					</thead>
					<tbody>
						<tr>
							<td class="border border-slate-300 text-gray-700 text-[12px] p-1">S</td>
							<td class="border border-slate-300 text-gray-700 text-[12px] p-1">C</td>
							<td class="border border-slate-300 text-gray-700 text-[12px] p-1">S</td>
						</tr>
					</tbody>
				</table>
				<p>{selected_school}</p>
			{:else}
				<ul class="flex gap-[2px] mt-2 mb-2 flex-wrap">
					<li class="min-w-[80px]">
						<button
							on:click={() => setPageTwoSub(1)}
							class={`text-[12px] w-full border border-slate-200 p-1 ${
								page_two_sub === 1 ? 'text-gray-600 bg-slate-300' : 'text-gray-600 border-gray-400'
							}`}>All</button
						>
					</li>
					<li class="min-w-[80px]">
						<button
							on:click={() => setPageTwoSub(2)}
							class={`text-[12px] w-full border border-slate-200 p-1 ${
								page_two_sub === 2 ? 'text-gray-600 bg-slate-300' : 'text-gray-600 border-gray-400'
							}`}>Eligibles</button
						>
					</li>
					<li class="min-w-[80px]">
						<button
							on:click={() => setPageTwoSub(3)}
							class={`text-[12px] w-full border border-slate-200 p-1 ${
								page_two_sub === 3 ? 'text-gray-600 bg-slate-300' : 'text-gray-600 border-gray-400'
							}`}>Ineligibles</button
						>
					</li>
				</ul>
				<table class="border-collapse border border-slate-400">
					<thead>
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
						<tr>
							<td class="border border-slate-300 text-gray-700 text-[12px] p-1">S</td>
							<td class="border border-slate-300 text-gray-700 text-[12px] p-1">C</td>
							<td class="border border-slate-300 text-gray-700 text-[12px] p-1">S</td>
							<td class="border border-slate-300 text-gray-700 text-[12px] p-1">Sumba</td>
						</tr>
					</tbody>
				</table>
				<p>{latest_status}</p>
			{/if}
		</div>
	</div>
</div>
