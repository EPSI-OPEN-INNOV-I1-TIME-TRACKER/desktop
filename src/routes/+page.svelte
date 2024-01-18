<script>
    import { invoke } from '@tauri-apps/api/tauri';
    import { onMount } from 'svelte';

    let activeWindowText = "No active window";

    async function fetchActiveWindowInfo() {
        try {
            const activeWindow = await invoke('get_active_window_info');
            alert(activeWindow);
            activeWindowText = activeWindow.app_name;
        } catch (error) {
            console.error('Error fetching active window info:', error);
        }
    }

    onMount(() => {
        const intervalId = setInterval(fetchActiveWindowInfo, 3000);
        return () => clearInterval(intervalId); // cleanup on component unmount
    });
</script>


<button on:click={fetchActiveWindowInfo}>Fetch active window info</button>

<p> {activeWindowText} </p>