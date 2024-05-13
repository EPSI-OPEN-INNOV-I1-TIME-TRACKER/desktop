<script lang="ts">
    import { invoke } from '@tauri-apps/api/tauri';
    import { onMount } from 'svelte';

    let activeWindowText: string = "No active window";
    let activeWindowHistory: string[] = []; // Pour stocker l'historique des fenêtres
    let timeSpent: object = {};
    let windowChangeCount: number = 0;
    let lastWindow: string | null = null;
    let showHistory: boolean = false; // État pour contrôler la visibilité de l'historique
    let appCount: Map<string, number> = new Map(); // Compteur pour chaque application
    let topApps: { name: string; count: number }[] = []; // Stocker le top 3 des applications

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
                activeWindowHistory.unshift(activeWindow.app_name); // Ajoute en tête de liste
                appCount.set(activeWindow.app_name, (appCount.get(activeWindow.app_name) || 0) + 1); // Incrémente le compteur
                updateTopApps();
            }
            const time: number = await invoke('get_current_window_time');
            timeSpent = formatDuration(time);
        } catch (error) {
            console.error('Error fetching active window info:', error);
        }
    }

    function updateTopApps() {
        topApps = Array.from(appCount.entries()) // Convertit Map en tableau
            .sort((a, b) => b[1] - a[1]) // Trie par nombre d'ouvertures
            .slice(0, 3) // Prend les trois premiers éléments
            .map((item, index) => ({ name: item[0], count: item[1] }));
    }

    function toggleHistory() {
        showHistory = !showHistory; // Bascule la visibilité de l'historique
    }

    function formatDuration(duration: number): any {
        return duration; // Implémenter selon le besoin
    }

    onMount(() => {
        const intervalId = setInterval(fetchActiveWindowInfo, 1000);
        return () => clearInterval(intervalId); // Cleanup
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
        <p>Time Spent: {JSON.stringify(timeSpent)}</p>
    </div>
    <div class="info-card">
        <p>Number of Window Switches: {windowChangeCount}</p>
    </div>
    <div class="info-card">
        <h3>Most Opened Apps</h3>
        {#each topApps as app, index}
            <button class="medal-button {index === 0 ? 'gold' : index === 1 ? 'silver' : 'bronze'}" on:click="{() => { /* handler for button click */ }}">
                {index === 0 ? '🥇' : index === 1 ? '🥈' : '🥉'} {app.name} ({app.count})
            </button>
        {/each}
    </div>
    <div class="info-card">
        <button on:click={toggleHistory}>Window History</button>
        {#if showHistory}
            <div class="history-content show">
                {#each activeWindowHistory as windowName}
                    <p>{windowName}</p>
                {/each}
            </div>
        {/if}
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
    .history-content {
        overflow-y: auto;
        max-height: 200px;
        display: none;
    }
    .show { /* Style pour montrer l'historique */
        display: block;
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