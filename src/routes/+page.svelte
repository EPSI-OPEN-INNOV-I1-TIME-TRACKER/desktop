<script>
    import { invoke } from '@tauri-apps/api/tauri';
    import { onMount } from 'svelte';

    let activeWindowText = "No active window";
    let activeWindowTime = "0";

    /**
	 * @param {number} seconds
	 */
    function formatTime(seconds) {
        // Fonction pour formater les secondes en un format lisible
        const h = Math.floor(seconds / 3600);
        const m = Math.floor((seconds % 3600) / 60);
        const s = seconds % 60;
        return `${h}h ${m}m ${s}s`;
    }

    async function fetchActiveWindowInfo() {
        try {
            const activeWindow = await invoke('get_active_window_info');
            activeWindowText = activeWindow.app_name;
            activeWindowTime = formatTime(activeWindow.active_time); // Mettre à jour le temps
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
<p>Time Active: {activeWindowTime}</p>
