<script lang="ts">
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/tauri';
  
	let timeSpentOnCurrentApp: number | null = null;
  
	async function getCurrentWindowTime() {
	  try {
		const duration: number = await invoke('get_current_window_time');
		timeSpentOnCurrentApp = duration;
	  } catch (error) {
		console.error('Failed to get current window time:', error);
		timeSpentOnCurrentApp = null;
	  }
	}
  
	onMount(() => {
	  getCurrentWindowTime();
	});
  </script>
  
  <main>
	<h1>Time Spent on Current App</h1>
	{#if timeSpentOnCurrentApp !== null}
	  <p>Time spent: {timeSpentOnCurrentApp} milliseconds</p>
	{:else}
	  <p>Unable to determine the time spent on the current app.</p>
	{/if}
  </main>
  
  <style>
	main {
	  text-align: center;
	  padding: 1rem;
	}
  </style>
  