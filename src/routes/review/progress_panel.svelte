<script>
	//@ts-nocheck
	import { onMount } from 'svelte';
	import Chart from 'chart.js/auto';
	import { get_pg_stat_byschool, get_child_ids_of } from '$lib/rust_functions';
	import { createEventDispatcher } from 'svelte';
	const dispatch = createEventDispatcher();

	const dispatchRefreshed = () => {
		dispatch('refreshed', {
			refreshed: true
		});
	};

	export let photographer = {
		id: 0,
		name: 'NOT SET'
	};
	export let school = 'NOT SET';
	export let collected_elig_ids = [];
	export let collected_inelig_ids = [];
	export let should_refresh = false;

	let school_stats = {
		num_elig: 0,
		num_inelig: 0
	};

	let incorect_ids = [];

	let photo_not_collected_stat = {
		num_elig: 0,
		num_inelig: 0
	};
	//CHART SETUPS
	const data = {
		labels: ['Eligible', 'Ineligible'],
		datasets: [
			{
				label: 'Collected',
				data: [0, 0],
				borderColor: 'rgb(75, 192, 192)',
				backgroundColor: 'rgba(75, 192, 192, 0.6)',
				borderWidth: 1,
				borderRadius: 1,
				borderSkipped: false
			},
			{
				label: 'Not Collected',
				data: [0, 0],
				borderColor: 'rgb(255, 205, 86)',
				backgroundColor: 'rgba(255, 205, 86, 0.6)',
				borderWidth: 1,
				borderRadius: 1,
				borderSkipped: false
			}
		]
	};
	let chart;
	//Refresh data on each photo change
	$: if (should_refresh) {
		get_pg_stat_byschool(photographer.id, school).then((res) => {
			console.log('backend resp:', res);
			school_stats.num_elig = res.num_elig;
			school_stats.num_inelig = res.num_inelig;

			get_child_ids_of(photographer.id, school).then((ids) => {
				const filtered_elig_ids = collected_elig_ids.filter((id) =>
					ids.elig_ids.includes(Number(id))
				);
				incorect_ids = collected_elig_ids.filter((id) => !ids.elig_ids.includes(Number(id)));
				photo_not_collected_stat = {
					num_elig: school_stats.num_elig - filtered_elig_ids.length,
					num_inelig: school_stats.num_inelig - collected_inelig_ids.length
				};
				console.log('Photo not collected stat', photo_not_collected_stat);
				//CHART SETUPS
				data.datasets[0].data = [collected_elig_ids.length, collected_inelig_ids.length];
				data.datasets[1].data = [
					photo_not_collected_stat.num_elig,
					photo_not_collected_stat.num_inelig
				];
				if (chart) {
					chart.update();
				}
				dispatchRefreshed();
			});
		});
	}

	onMount(async () => {
		let ctx;
		ctx = document.getElementById('myChart');

		const DATA_COUNT = 2;
		const NUMBER_CFG = { count: DATA_COUNT, min: 0, max: 300 };

		const config = {
			type: 'bar',
			data: data,
			options: {
				responsive: true,
				scales: {
					x: {
						stacked: true
					},
					y: {
						stacked: true
					}
				},
				plugins: {
					legend: {
						position: 'top'
					},
					title: {
						display: true,
						text: 'Photo Collecting Progress in This School'
					}
				}
			}
		};

		chart = new Chart(ctx, { ...config });
	});
</script>

<section class="p-2">
	<h3>Photographer progress</h3>
	<div class="flex justify-between">
		<p class="text-sm text-gray-600">Photographer</p>
		<p class="text-sm font-bold">{photographer.name}</p>
	</div>
	<div class="flex justify-between">
		<p class="text-sm text-gray-600">School</p>
		<p class="text-sm font-bold">{school}</p>
	</div>
	<div class="flex justify-between">
		<p class="text-sm text-gray-600">All Eligible</p>
		<p class="text-sm">{school_stats.num_elig}</p>
	</div>
	<div class="flex justify-between">
		<p class="text-sm text-gray-600">All Inligible</p>
		<p class="text-sm">{school_stats.num_inelig}</p>
	</div>
	<div class="flex justify-between">
		<p class="text-sm text-gray-600">Collected Eligible</p>
		<p class="text-sm font-bold">{collected_elig_ids.length}</p>
	</div>
	<div class="flex justify-between">
		<p class="text-sm text-gray-600">Collected Ineligle</p>
		<p class="text-sm font-bold">{collected_inelig_ids.length}</p>
	</div>
	<div class="flex justify-between">
		<p class="text-sm text-gray-600">NOT Collected Eligible</p>
		<p class="text-sm font-bold">{photo_not_collected_stat.num_elig}</p>
	</div>
	<section class="mt-16">
		<div>
			<canvas id="myChart" />
		</div>
		<section class="mt-8">
			<h3 class="text-gray-400 text-sm">Unknown or misplaced photo ids</h3>
			<div class="flex flex-wrap gap-1">
				{#each incorect_ids as child_id, idx ('chid' + idx)}
					<span class="block text-[12px] text-slate-500 border p-1">{child_id}</span>
				{/each}
			</div>
		</section>
	</section>
</section>
