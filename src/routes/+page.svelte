<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { onMount } from 'svelte';

	let activeWindowText: string = 'No active window';
	let timeSpent: string = "";

	interface ActiveWindow {
		app_name: string;
	}

	async function fetchActiveWindowInfo(): Promise<void> {
		try {
			const activeWindow: ActiveWindow = await invoke('get_active_window_info');
			activeWindowText = activeWindow.app_name;

			const time: number = await invoke('get_current_window_time');
            timeSpent = formatDuration(time);
		} catch (error) {
			console.error('Error fetching active window info:', error);
		}
	}

	function formatDuration(duration: number): string {
		const seconds = Math.floor((duration / 1000) % 60);
		const minutes = Math.floor((duration / (1000 * 60)) % 60);
		const hours = Math.floor((duration / (1000 * 60 * 60)) % 24);

		return `${hours}h ${minutes}m ${seconds}s`;
	}

	onMount(() => {
		const intervalId = setInterval(fetchActiveWindowInfo, 1000);
		return () => clearInterval(intervalId); // cleanup on component unmount
	});
</script>

<button on:click={fetchActiveWindowInfo}>Fetch active window info</button>

<p>Active Window: {activeWindowText}</p>
<p>Time Spent: {timeSpent}</p>
