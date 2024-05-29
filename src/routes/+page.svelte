<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/tauri';

  interface AppUsage {
    app_name: string;
    window_id: number;
    duration: number; // Duration in seconds
  }

  let trackedApps: AppUsage[] = [];

  async function fetchTrackedApps() {
    try {
      const result: AppUsage[] = await invoke('get_tracked_apps');
      trackedApps = result;
    } catch (error) {
      console.error("Failed to fetch tracked apps", error);
    }
  }

  onMount(() => {
    fetchTrackedApps();
  });
</script>

<main class="p-4">
  <h1 class="text-2xl font-bold mb-4">App Usage Tracker</h1>
  <button class="mb-4 p-2 bg-blue-500 text-white rounded" on:click={fetchTrackedApps}>Refresh</button>
  <table class="table-auto w-full border-collapse border border-gray-200">
    <thead>
      <tr class="bg-gray-100">
        <th class="border p-2">App Name</th>
        <th class="border p-2">Window ID</th>
        <th class="border p-2">Duration (seconds)</th>
      </tr>
    </thead>
    <tbody>
      {#each trackedApps as app}
        <tr>
          <td class="border p-2">{app.app_name}</td>
          <td class="border p-2">{app.window_id}</td>
          <td class="border p-2">{app.duration}</td>
        </tr>
      {/each}
    </tbody>
  </table>
</main>
