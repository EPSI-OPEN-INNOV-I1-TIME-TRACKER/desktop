<script lang="ts">
    import { onMount } from 'svelte';
    import { invoke } from '@tauri-apps/api/tauri';
    import { writable } from 'svelte/store';
    import AppTable from './AppTable.svelte';
    import ControlPanel from './ControlPanel.svelte';
	import type { AppUsage } from '$lib/types';
  
    let trackedApps = writable<AppUsage[]>([]);
    let autoRefresh = writable<boolean>(false);
    let intervalId: NodeJS.Timeout | null = null;
  
    async function fetchTrackedApps() {
      try {
        const result: AppUsage[] = await invoke('get_tracked_apps');
        trackedApps.set(result);
      } catch (error) {
        console.error("Failed to fetch tracked apps", error);
      }
    }
  
    function toggleAutoRefresh() {
      autoRefresh.update((value) => {
        if (!value) {
          intervalId = setInterval(fetchTrackedApps, 500);
        } else {
          if (intervalId) clearInterval(intervalId);
          intervalId = null;
        }
        return !value;
      });
    }
  
    onMount(() => {
      fetchTrackedApps();
      return () => {
        if (intervalId) clearInterval(intervalId);
      };
    });
  </script>
  
  <main class="p-4">
    <h1 class="text-2xl font-bold mb-4">App Usage Tracker</h1>
    <ControlPanel {autoRefresh} onManualRefresh={fetchTrackedApps} onToggleAutoRefresh={toggleAutoRefresh} />
    <AppTable {trackedApps} />
  </main>
  
  <style>
    main {
      padding: 1em;
      font-family: Arial, sans-serif;
    }
  
    .text-2xl {
      font-size: 1.5rem;
    }
  
    .font-bold {
      font-weight: bold;
    }
  
    .mb-4 {
      margin-bottom: 1rem;
    }
  
    .p-4 {
      padding: 1rem;
    }
  </style>
  