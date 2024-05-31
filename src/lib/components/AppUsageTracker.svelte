<script lang="ts">
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	import { writable } from 'svelte/store';
	import AppTable from './AppTable.svelte';
	import ControlPanel from './ControlPanel.svelte';
	import Graph from './Graph.svelte';
	import type { AppUsage } from '$lib/types';

	let trackedApps = writable<AppUsage[]>([]);
	let autoRefresh = writable<boolean>(false);
	let intervalId: NodeJS.Timeout | null = null;

	let graphData = writable({
		labels: ['App 1', 'App 2', 'App 3', 'App 4'],
		datasets: [
			{
				label: 'App Usage Duration (seconds)',
				data: [30, 60, 90, 120],
				backgroundColor: [
					'rgba(75, 192, 192, 0.2)',
					'rgba(75, 192, 192, 0.2)',
					'rgba(75, 192, 192, 0.2)',
					'rgba(75, 192, 192, 0.2)'
				],
				borderColor: [
					'rgba(75, 192, 192, 1)',
					'rgba(75, 192, 192, 1)',
					'rgba(75, 192, 192, 1)',
					'rgba(75, 192, 192, 1)'
				],
				borderWidth: 1
			}
		]
	});

	async function fetchTrackedApps() {
		try {
			const result: AppUsage[] = await invoke('get_tracked_apps');
			trackedApps.set(result);

			const topApps = result.sort((a, b) => b.duration - a.duration).slice(0, 5);

			const labels = topApps.map((app) => app.app_name);
			const data = topApps.map((app) => app.duration);
			const backgroundColors = [
				'rgba(255, 99, 132, 0.2)',
				'rgba(54, 162, 235, 0.2)',
				'rgba(255, 206, 86, 0.2)',
				'rgba(75, 192, 192, 0.2)',
				'rgba(153, 102, 255, 0.2)'
			];
			const borderColors = [
				'rgba(255, 99, 132, 1)',
				'rgba(54, 162, 235, 1)',
				'rgba(255, 206, 86, 1)',
				'rgba(75, 192, 192, 1)',
				'rgba(153, 102, 255, 1)'
			];

			graphData.set({
				labels,
				datasets: [
					{
						label: 'App Usage Duration (seconds)',
						data,
						backgroundColor: backgroundColors,
						borderColor: borderColors,
						borderWidth: 1
					}
				]
			});
		} catch (error) {
			console.error('Failed to fetch tracked apps', error);
		}
	}

	function toggleAutoRefresh() {
		autoRefresh.update((value) => {
			if (!value) {
				intervalId = setInterval(fetchTrackedApps, 500);
			} else {
				if (intervalId) clearInterval(intervalId);
				intervalId = null;
			}
			return !value;
		});
	}

	onMount(() => {
		fetchTrackedApps();
		return () => {
			if (intervalId) clearInterval(intervalId);
		};
	});
</script>

<main class="p-4">
	<h1 class="mb-4 text-2xl font-bold">App Usage Tracker</h1>
	<h2 class="mb-4 mt-4 text-xl font-bold">App Usage Graph</h2>
	<ControlPanel
		{autoRefresh}
		onManualRefresh={fetchTrackedApps}
		onToggleAutoRefresh={toggleAutoRefresh}
	/>
	<Graph {graphData} />
	<h2 class="mb-4 mt-4 text-xl font-bold">Tracked Apps</h2>
	<AppTable {trackedApps} />
</main>

<style>
	main {
		padding: 1em;
		font-family: Arial, sans-serif;
	}

	.text-2xl {
		font-size: 1.5rem;
	}

	.font-bold {
		font-weight: bold;
	}

	.mb-4 {
		margin-bottom: 1rem;
	}

	.p-4 {
		padding: 1rem;
	}

	.mt-4 {
		margin-top: 1rem;
	}

	.text-xl {
		font-size: 1.25rem;
	}
</style>