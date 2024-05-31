<script lang="ts">
	import { writable } from 'svelte/store';

	export let autoRefresh = writable<boolean>(false);
	export let onManualRefresh: () => void;
	export let onToggleAutoRefresh: () => void;
</script>

<div class="mb-4 flex items-center">
	<button class="mr-2 rounded bg-blue-500 p-2 text-white" on:click={onManualRefresh}
		>Manual Refresh</button
	>
	<button class="rounded bg-green-500 p-2 text-white" on:click={onToggleAutoRefresh}>
		{#if $autoRefresh}
			Disable Auto-Refresh
		{:else}
			Enable Auto-Refresh
		{/if}
	</button>
</div>

<style>
	.flex {
		display: flex;
	}

	.items-center {
		align-items: center;
	}

	.mb-4 {
		margin-bottom: 1rem;
	}

	.p-2 {
		padding: 0.5rem;
	}

	.bg-blue-500 {
		background-color: #4299e1;
	}

	.bg-green-500 {
		background-color: #48bb78;
	}

	.text-white {
		color: white;
	}

	.rounded {
		border-radius: 0.25rem;
	}

	.mr-2 {
		margin-right: 0.5rem;
	}
</style>
