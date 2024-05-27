<script lang="ts">
    import { invoke } from '@tauri-apps/api/tauri';
    import { onMount } from 'svelte';
  
    let openWindows: WindowInfo[] = [];
  
    interface WindowInfo {
      title: string;
      app_name: string;
    }
  
    async function fetchOpenWindows(): Promise<void> {
      try {
        openWindows = await invoke('get_open_windows');
      } catch (error) {
        console.error('Error fetching open windows:', error);
      }
    }
  
    onMount(() => {
      fetchOpenWindows();
      const intervalId = setInterval(fetchOpenWindows, 10000); // Actualiser toutes les 10 secondes
      return () => clearInterval(intervalId);
    });
  </script>
  
  <style>
    /* Styles spécifiques si nécessaire */
  </style>
  
  <div>
    <h3>Open Windows</h3>
    <ul>
      {#each openWindows as window}
        <li>{window.title} - {window.app_name}</li>
      {/each}
    </ul>
  </div>  