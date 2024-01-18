<script lang="ts">
    import { invoke } from '@tauri-apps/api/tauri';
    import { onMount } from 'svelte';

    let activeWindowText: string = "No active window";
    let timeSpent: object = {};

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

    function formatDuration(duration: number): any {
        // Convert duration in milliseconds to a readable format
        // Implement this function based on how you want to format the duration
        // For example, convert to hours, minutes, and seconds
        // return `${duration}ms`; // Placeholder, replace with actual formatting
        return duration
    }

    onMount(() => {
        const intervalId = setInterval(fetchActiveWindowInfo, 1000);
        return () => clearInterval(intervalId); // cleanup on component unmount
    });
</script>

<button on:click={fetchActiveWindowInfo}>Fetch active window info</button>

<p>Active Window: {activeWindowText}</p>
<p>Time Spent: {JSON.stringify(timeSpent)}</p>
