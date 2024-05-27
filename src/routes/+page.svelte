<script lang="ts">
    import { invoke } from '@tauri-apps/api/tauri';
    import { onMount } from 'svelte';
    import Chart from 'chart.js/auto';
    import './styles.css';  // Importer le fichier CSS
  
    let activeWindowText: string = "No active window";
    let activeWindowHistory: string[] = [];
    let windowChangeCount: number = 0;
    let lastWindow: string | null = null;
    let lastCheckTime: number = Date.now();
    let showHistory: boolean = false;
    let appTime: Map<string, number> = new Map();
    let topApps: { name: string; time: TimeSpent }[] = [];
    let windowChangeData: number[] = Array(24).fill(0); // Données pour le graphique
    let timeSpent: TimeSpent = { hours: 0, minutes: 0, seconds: 0 };
    let rankingPeriod: string = 'month'; // 'month' ou 'year'
  
    interface ActiveWindow {
      app_name: string;
    }
  
    interface TimeSpent {
      hours: number;
      minutes: number;
      seconds: number;
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
          updateWindowChangeData(); // Met à jour les données du graphique
        }
  
        const elapsedTime = now - lastCheckTime;
        lastCheckTime = now;
        appTime.set(activeWindow.app_name, (appTime.get(activeWindow.app_name) || 0) + elapsedTime);
        updateTopApps();
  
        const time: number = await invoke('get_current_window_time');
        if (typeof time !== "number" || isNaN(time)) {
          console.error("Invalid time value received:", time);
          return;
        }
        timeSpent = formatDuration(time);
      } catch (error) {
        console.error('Error fetching active window info:', error);
      }
    }
  
    function updateWindowChangeData() {
      const now = new Date();
      const hour = now.getHours();
      windowChangeData[hour]++;
      if (windowChangeChart) {
        windowChangeChart.update();
      }
    }
  
    function updateTopApps() {
      topApps = Array.from(appTime.entries())
        .sort((a, b) => b[1] - a[1])
        .slice(0, 3)
        .map((item) => ({ name: item[0], time: formatDuration(item[1]) }));
    }
  
    function formatDuration(duration: number): TimeSpent {
      const seconds = Math.floor((duration / 1000) % 60);
      const minutes = Math.floor((duration / (1000 * 60)) % 60);
      const hours = Math.floor((duration / (1000 * 60 * 60)) % 24);
  
      return { hours, minutes, seconds };
    }
  
    function getMedalEmoji(index: number): string {
      if (index === 0) return '🥇';
      if (index === 1) return '🥈';
      if (index === 2) return '🥉';
      return '';
    }
  
    let windowChangeChart: Chart<"bar", number[], string>;
  
    function initializeChart() {
      const canvas = document.getElementById('windowChangeChart') as HTMLCanvasElement | null;
      if (canvas) {
        const ctx = canvas.getContext('2d');
        if (ctx) {
          windowChangeChart = new Chart(ctx, {
            type: 'bar',
            data: {
              labels: Array.from({ length: 24 }, (_, i) => `${i}h`),
              datasets: [{
                label: 'Window Changes per Hour',
                data: windowChangeData,
                backgroundColor: 'rgba(75, 192, 192, 0.2)',
                borderColor: 'rgba(75, 192, 192, 1)',
                borderWidth: 1
              }]
            },
            options: {
              scales: {
                y: {
                  beginAtZero: true
                }
              }
            }
          });
        }
      }
    }
  
    onMount(() => {
      const intervalId = setInterval(fetchActiveWindowInfo, 1000);
      initializeChart();
      return () => clearInterval(intervalId);
    });
  
    function toggleHistory() {
      showHistory = !showHistory;
    }
  
    function handleRankingPeriodChange(event: Event) {
      const selectElement = event.target as HTMLSelectElement;
      if (selectElement) {
        rankingPeriod = selectElement.value;
      }
    }
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
    <div class="info-card chart-card">
      <canvas id="windowChangeChart"></canvas>
    </div>
    <div class="info-card">
      <h3>Most Opened Apps</h3>
      {#each topApps as app, index}
        <button class="medal-button {index === 0 ? 'gold' : index === 1 ? 'silver' : 'bronze'}">
          {getMedalEmoji(index)} {app.name} ({app.time.hours}h {app.time.minutes}m {app.time.seconds}s)
        </button>
      {/each}
    </div>
    <div class="info-card">
      <h3>Ranking for {rankingPeriod}</h3>
      <select on:change={handleRankingPeriodChange}>
        <option value="month">Month</option>
        <option value="year">Year</option>
      </select>
      <!-- Ajoute ici le contenu du classement basé sur la période choisie -->
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