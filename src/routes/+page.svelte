<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/tauri';
  import { writable } from 'svelte/store';

  interface AppUsage {
    app_name: string;
    window_id: number;
    duration: number; // Duration in seconds
  }

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
  <div class="flex items-center mb-4">
    <button class="p-2 bg-blue-500 text-white rounded mr-2" on:click={fetchTrackedApps}>Manual Refresh</button>
    <button class="p-2 bg-green-500 text-white rounded" on:click={toggleAutoRefresh}>
      {#if $autoRefresh} Disable Auto-Refresh
      {:else} Enable Auto-Refresh
      {/if}
    </button>
  </div>
  <table class="table-auto w-full border-collapse border border-gray-200">
    <thead>
      <tr class="bg-gray-100">
        <th class="border p-2">App Name</th>
        <th class="border p-2">Window ID</th>
        <th class="border p-2">Duration (seconds)</th>
      </tr>
    </thead>
    <tbody>
      {#each $trackedApps as app}
        <tr>
          <td class="border p-2">{app.app_name}</td>
          <td class="border p-2">{app.window_id}</td>
          <td class="border p-2">{app.duration}</td>
        </tr>
      {/each}
    </tbody>
  </table>
</main>

<style>
  main {
    padding: 1em;
    font-family: Arial, sans-serif;
  }

  table {
    width: 100%;
    border-collapse: collapse;
  }

  th, td {
    padding: 0.5em;
    border: 1px solid #ccc;
  }

  th {
    background-color: #f4f4f4;
  }
</style>
