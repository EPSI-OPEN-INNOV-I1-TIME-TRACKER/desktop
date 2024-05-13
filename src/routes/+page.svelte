<script lang="ts">
    import { invoke } from '@tauri-apps/api/tauri';
    import { onMount } from 'svelte';

    let activeWindowText: string = "No active window";
    let timeSpent: object = {};
    let windowChangeCount: number = 0;
    let lastWindow: string | null = null;

    interface ActiveWindow {
        app_name: string;
    }

    async function fetchActiveWindowInfo(): Promise<void> {
        try {
            const activeWindow: ActiveWindow = await invoke('get_active_window_info');
            
            if (lastWindow !== activeWindow.app_name) {
                lastWindow = activeWindow.app_name;
                windowChangeCount++;
                activeWindowText = activeWindow.app_name;
            }

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
        return duration;
    }

    onMount(() => {
        const intervalId = setInterval(fetchActiveWindowInfo, 1000);
        return () => clearInterval(intervalId); // cleanup on component unmount
    });
</script>

<style>
    .info-container {
        display: flex;
        flex-direction: column;
        align-items: flex-start;
        padding: 20px;
        gap: 10px;
        position: fixed;
        left: 10px;
        top: 10px;
        width: 200px;
    }
    .info-card {
        background: #f3f4f6;
        border: 1px solid #d1d5db;
        border-radius: 8px;
        padding: 10px;
        box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
    }
    button {
        width: 100%;
        padding: 8px;
        border-radius: 8px;
        background-color: #4f46e5;
        color: white;
        border: none;
        cursor: pointer;
    }
    button:hover {
        background-color: #4338ca;
    }
</style>

<div class="info-container">
    <div class="info-card">
        <button on:click={fetchActiveWindowInfo}>Fetch active window info</button>
    </div>
    <div class="info-card">
        <p>Active Window: {activeWindowText}</p>
    </div>
    <div class="info-card">
        <p>Time Spent: {JSON.stringify(timeSpent)}</p>
    </div>
    <div class="info-card">
        <p>Number of Window Switches: {windowChangeCount}</p>
    </div>
</div>