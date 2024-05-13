<script lang="ts">
    import { invoke } from '@tauri-apps/api/tauri';
    import { onMount } from 'svelte';

    let activeWindowText: string = "No active window";
    let activeWindowHistory: string[] = [];
    let windowChangeCount: number = 0;
    let lastWindow: string | null = null;
    let lastCheckTime: number = Date.now();
    let showHistory: boolean = false;
    let appTime: Map<string, number> = new Map();
    let topApps: { name: string; time: TimeSpent }[] = [];

    interface ActiveWindow {
        app_name: string;
    }

    async function fetchActiveWindowInfo(): Promise<void> {
        try {
            const now = Date.now();
            const activeWindow: ActiveWindow = await invoke('get_active_window_info');
            if (lastWindow !== activeWindow.app_name) {
                lastWindow = activeWindow.app_name;
                windowChangeCount++;
                activeWindowText = activeWindow.app_name;
                activeWindowHistory.unshift(activeWindow.app_name);
            }

            const elapsedTime = now - lastCheckTime;
            lastCheckTime = now;
            appTime.set(activeWindow.app_name, (appTime.get(activeWindow.app_name) || 0) + elapsedTime);
            updateTopApps();

            const time: number = await invoke('get_current_window_time');
            console.log("Time from API:", time); // Log pour voir la valeur retournée
            if (typeof time !== "number" || isNaN(time)) {
                console.error("Invalid time value received:", time);
                return; // Sortie anticipée en cas de valeur non valide
            }
            timeSpent = formatDuration(time);
        } catch (error) {
            console.error('Error fetching active window info:', error);
        }
    }

    function updateTopApps() {
        topApps = Array.from(appTime.entries())
            .sort((a, b) => b[1] - a[1])
            .slice(0, 3)
            .map((item, index) => ({ name: item[0], time: formatDuration(item[1]) }));
    }

    interface TimeSpent {
        hours: number;
        minutes: number;
        seconds: number;
    }

    let timeSpent: TimeSpent = { hours: 0, minutes: 0, seconds: 0 }; // Initialisation avec des valeurs par défaut

    function formatDuration(duration: number): TimeSpent {
        const seconds = Math.floor((duration / 1000) % 60);
        const minutes = Math.floor((duration / (1000 * 60)) % 60);
        const hours = Math.floor((duration / (1000 * 60 * 60)) % 24);

        return { hours, minutes, seconds }; // Renvoie un objet conforme à l'interface TimeSpent
    }

    onMount(() => {
        const intervalId = setInterval(fetchActiveWindowInfo, 1000);
        return () => clearInterval(intervalId);
    });
</script>


<div class="info-container">
    <div class="info-card">
        <button on:click={fetchActiveWindowInfo}>Fetch active window info</button>
    </div>
    <div class="info-card">
        <p>Active Window: {activeWindowText}</p>
    </div>
    <div class="info-card">
        <p>Time Spent: {timeSpent.hours} hours {timeSpent.minutes} minutes {timeSpent.seconds} seconds</p>
    </div>
    <div class="info-card">
        <p>Number of Window Switches: {windowChangeCount}</p>
    </div>
    <div class="info-card">
        <h3>Most Opened Apps</h3>
            {#each topApps as app, index}
                <button class="medal-button {index === 0 ? 'gold' : index === 1 ? 'silver' : 'bronze'}" on:click="{() => { /* handler for button click */ }}">
                    {index === 0 ? '🥇' : index === 1 ? '🥈' : '🥉'} {app.name} ({app.time.hours}h {app.time.minutes}m {app.time.seconds}s)
                </button>
            {/each}
    </div>
    
</div>


<style>
    .info-container {
        display: flex;
        flex-direction: column;
        align-items: center;
        padding: 20px;
        gap: 10px;
        position: fixed;
        left: 10%;
        right: 10%;
        top: 10px;
        width: 80%;
    }
    .info-card {
        background: #f3f4f6;
        border: 1px solid #d1d5db;
        border-radius: 8px;
        padding: 10px;
        box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
        width: 100%;
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
    .gold {
        background-color: rgba(255, 215, 0, 0.7);
        color: black; /* Texte en noir pour la médaille d'or */
    }
    .silver {
        background-color: rgba(192, 192, 192, 0.7);
        color: black; /* Texte en noir pour la médaille d'argent */
    }
    .bronze {
        background-color: rgba(205, 127, 50, 0.7);
        color: black; /* Texte en noir pour la médaille de bronze */
    }
</style>