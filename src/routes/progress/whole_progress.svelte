<script>
	//@ts-nocheck
	import { get_all_photographers, get_pg_stats_all, get_all_child_ids } from '$lib/rust_functions';
	import { onDestroy, onMount } from 'svelte';
	import Chart from 'chart.js/auto';
	import { mainDestPath } from '../../lib/store';

	//setup paths
	let main_dest_path = '';
	const unsub_mainDestPath = mainDestPath.subscribe((value) => (main_dest_path = value));
	onDestroy(unsub_mainDestPath);
	onMount(async () => {
		let ctx = document.getElementById('myChart');

		new Promise(async (resolve, reject) => {
			let photographers = await get_all_photographers();
			//console.log('All Photographers:', photographers);
			const pg_stats = [];
			photographers.forEach(async (pg) => {
				get_pg_stats_all(pg.id).then(async (pg_stat) => {
					let total_elig = pg_stat.reduce((acc, nextItem) => acc + nextItem.num_elig, 0);
					let total_inelig = pg_stat.reduce((acc, nextItem) => acc + nextItem.num_inelig, 0);
					let schools = pg_stat.map((stat) => stat.school);
					//console.log(`schools for ${pg.name}: ${schools}`);

					const getTotalElig = (some_schools) => {
						return new Promise((res) => {
							let total = 0;
							some_schools.forEach((school, idx) => {
								get_all_child_ids(main_dest_path + school).then((child_ids) => {
									total += child_ids.length;
									if (idx === some_schools.length - 1) {
										res(total);
									}
								});
							});
						});
					};
					const getTotalInelig = (some_schools) => {
						return new Promise((res) => {
							let total = 0;
							some_schools.forEach((school, idx) => {
								get_all_child_ids(main_dest_path + 'Ineligibles\\' + school).then((child_ids) => {
									total += child_ids.length;
									if (idx === some_schools.length - 1) {
										res(total);
									}
								});
							});
						});
					};

					let total_collected_elig = await getTotalElig(schools);
					let total_collected_inelig = await getTotalInelig(schools);

					console.log(
						`Collected by ${pg.name}: Elig ${total_collected_elig}, Inelig ${total_collected_inelig}`
					);

					const _pg_stat = {
						id: pg.id,
						name: pg.name,
						total_elig: total_elig,
						total_inelig: total_inelig,
						total_collected_elig: total_collected_elig,
						total_collected_inelig: total_collected_inelig
					};
					pg_stats.push(_pg_stat);

					if (pg_stats.length === photographers.length) {
						//console.log('pg_stats FINAL:', pg_stats);
						resolve(pg_stats);
					}
				});
			});
		})
			.then((pg_stats) => {
				console.log('RESOLVED pg stats:', pg_stats);
				// pg_stats.forEach((stat) => console.log('Stat:', stat));

				//CHART SETUPS
				const data = {
					labels: pg_stats.map((pg) => pg.name),
					datasets: [
						{
							label: 'Collected Eligible',
							data: pg_stats.map((pg) => pg.total_collected_elig),
							borderColor: 'rgb(75, 220, 152)',
							backgroundColor: 'rgba(75, 220, 152, 0.6)',
							borderWidth: 1,
							borderRadius: 1,
							borderSkipped: false,
							stack: 'Stack 0'
						},
						{
							label: 'NOT COLLECTED Eligible',
							data: pg_stats.map((pg) => pg.total_elig - pg.total_collected_elig),
							borderColor: 'rgb(255, 205, 86)',
							backgroundColor: 'rgba(255, 205, 86, 0.6)',
							borderWidth: 1,
							borderRadius: 1,
							borderSkipped: false,
							stack: 'Stack 0'
						},
						{
							label: 'Collected Ineligible',
							data: pg_stats.map((pg) => pg.total_collected_inelig),
							borderColor: 'rgb(75, 202, 232)',
							backgroundColor: 'rgba(75, 202, 255, 0.6)',
							borderWidth: 1,
							borderRadius: 1,
							borderSkipped: false,
							stack: 'Stack 1'
						},
						{
							label: 'NOT COLLECTED Ineligible',
							data: pg_stats.map((pg) => pg.total_inelig - pg.total_collected_inelig),
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
								text: 'All Photo Collecting Progress'
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

				let chart = new Chart(ctx, { ...config });
			})
			.catch((err) => console.log('Failed: ', err));
	});
</script>

<div class="border w-full">
	<canvas id="myChart" />
</div>
