<script lang="ts">
	import type { AppUsage } from '$lib/types';
	import { writable } from 'svelte/store';

	export let trackedApps = writable<AppUsage[]>([]);
</script>

<table class="w-full table-auto border-collapse border border-gray-200">
	<thead>
		<tr class="bg-gray-100">
			<th class="border p-2">App Name</th>
			<th class="border p-2">Window ID</th>
			<th class="border p-2">Duration (seconds)</th>
		</tr>
	</thead>
	<tbody>
		{#each $trackedApps as app}
			<tr>
				<td class="border p-2">{app.app_name}</td>
				<td class="border p-2">{app.window_id}</td>
				<td class="border p-2">{app.duration}</td>
			</tr>
		{/each}
	</tbody>
</table>

<style>
	table {
		width: 100%;
		border-collapse: collapse;
	}

	th,
	td {
		padding: 0.5em;
		border: 1px solid #ccc;
	}

	th {
		background-color: #f4f4f4;
	}
</style>
