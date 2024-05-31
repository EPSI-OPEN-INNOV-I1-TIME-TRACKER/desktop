<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { onMount } from 'svelte';
	import Icon from '@iconify/svelte';
	import Modal from './Modal.svelte';
	import Switch from './Switch.svelte';
	import ToggleList from './ToggleList.svelte';
	import BarChart from './BarChart.svelte';
	import Chart from 'chart.js/auto';

	let apps: string[] = [];
	let timeSpentToday: string = '00:00:00';
	let timeSpentSelectedApp: string = '00:00:00';
	let connectedUser: string = 'John Doe';
	let sliderValue: any;

	let activeWindowText: string = 'No active window';
	let activeWindowHistory: string[] = [];
	let windowChangeCount: number = 0;
	let lastWindow: string | null = null;
	let lastCheckTime: number = Date.now();
	let appTime: Map<string, number> = new Map();
	let topApps: { name: string; time: TimeSpent }[] = [];
	let windowChangeData: number[] = Array(24).fill(0); // Data for the chart
	let timeSpent: TimeSpent = { hours: 0, minutes: 0, seconds: 0 };

	let showModal = false;
	let modalTitle: string = ''; // Define the modalTitle variable
	let modalType: string = ''; // Define the modalType variable

	let activeApp: string = ''; // Declare activeApp here

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
				updateWindowChangeData(); // Update chart data

				// Update app list
				if (!apps.includes(activeWindow.app_name)) {
					apps = [activeWindow.app_name, ...apps];
				}
			}

			const elapsedTime = now - lastCheckTime;
			lastCheckTime = now;
			appTime.set(activeWindow.app_name, (appTime.get(activeWindow.app_name) || 0) + elapsedTime);
			updateTopApps();

			const time: number = await invoke('get_current_window_time');
			if (typeof time !== 'number' || isNaN(time)) {
				console.error('Invalid time value received:', time);
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

	function handleAppClose(appName: string) {
		apps = apps.filter((app) => app !== appName);
	}

	function handleClearApps() {
		apps = [];
	}

	let windowChangeChart: Chart<'bar', number[], string>;

	function initializeChart() {
		const canvas = document.getElementById('windowChangeChart') as HTMLCanvasElement | null;
		if (canvas) {
			const ctx = canvas.getContext('2d');
			if (ctx) {
				windowChangeChart = new Chart(ctx, {
					type: 'bar',
					data: {
						labels: Array.from({ length: 24 }, (_, i) => `${i}h`),
						datasets: [
							{
								label: 'Window Changes per Hour',
								data: windowChangeData,
								backgroundColor: 'rgba(75, 192, 192, 0.2)',
								borderColor: 'rgba(75, 192, 192, 1)',
								borderWidth: 1
							}
						]
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

	function openSettingsModal() {
		modalTitle = '';
		modalType = 'settings';
		showModal = true;
	}

	function openUserModal() {
		modalTitle = '';
		modalType = 'user';
		showModal = true;
	}

	function closeModal() {
		showModal = false;
	}

	function handleToggle(app: string) {
		activeApp = activeApp === app ? '' : app; // Update activeApp state
	}

	onMount(() => {
		const intervalId = setInterval(fetchActiveWindowInfo, 1000);
		initializeChart();
		return () => clearInterval(intervalId);
	});
</script>

<div id="app">
	<header>
		<div id="left-section">
			<Icon icon="mdi:clock" width="48" height="48" />
		</div>
		<div id="middle-section">
			<h1>TIME'TRACKER</h1>
		</div>
		<div id="right-section">
			<button class="header-btn" on:click={openSettingsModal}>
				<Icon icon="mdi:settings" width="48" height="48" />
			</button>
			<button class="header-btn" on:click={openUserModal}>
				<Icon icon="mdi:user" width="48" height="48" />
			</button>
		</div>
	</header>

	<main>
		<div id="left-side-menu">
			<ToggleList {apps} {activeApp} onToggle={handleToggle} onClear={handleClearApps} />
		</div>

		<div id="tracking">
			<BarChart />
		</div>

		<div>
			<!-- Insert the section with the most opened apps (podium) -->
		</div>
	</main>

	<footer>
		<div class="time-counter">
			<h4>TEMPS PC AUJOURD'HUI :</h4>
			<div class="time-counter-display">
				<p>{timeSpentToday}</p>
			</div>
		</div>

		<div class="time-counter">
			<h4>TEMPS D'APPLICATION SELECTIONNEE :</h4>
			<div class="time-counter-display">
				<p>{timeSpentSelectedApp}</p>
			</div>
		</div>
	</footer>

	{#if showModal}
		<Modal {modalTitle} on:close={closeModal}>
			{#if modalType === 'settings'}
				<div>
					<h2>Paramètres</h2>
					<Switch
						bind:value={sliderValue}
						label="Activer les notifications"
						fontSize={14}
						design="slider"
					/>
				</div>
			{:else if modalType === 'user'}
				<div>
					<h2>Vous êtes connecté en tant que...</h2>
					<h3>{connectedUser}</h3>
				</div>
			{/if}
		</Modal>
	{/if}
</div>

<style>
	* {
		margin: 0;
		padding: 0;
		box-sizing: border-box;
		font-family: Arial, sans-serif;
	}

	:global(body) {
		background-color: #e4e1dc;
	}

	#app {
		display: flex;
		flex-direction: column;
		height: 100vh;
	}

	header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		background-color: #93441a;
		color: #eee6d8;
		padding: 30px 60px;
		border-radius: 10px 10px 10px 10px;
	}

	.header-btn {
		background-color: transparent;
		color: #eee6d8;
		border: none;
		cursor: pointer;
	}

	main {
		display: flex;
		flex-grow: 1;
	}

	#left-side-menu {
		background-color: #b67332;
		border-radius: 10px 10px 10px 10px;
		width: 200px;
		padding: 20px;
		margin: 10px;
		display: flex;
		flex-direction: column;
		justify-content: space-between;
	}

	#tracking {
		flex-grow: 1;
		padding: 20px;
		margin: 20px;
		border-radius: 8px;
		border: 5px double #804000;
		background-color: #ffffff;
	}

	footer {
		display: flex;
		justify-content: space-around;
		padding: 30px 60px;
		border-radius: 10px 10px 10px 10px;
		background-color: #804000;
		color: #eee6d8;
	}

	footer .time-counter {
		display: flex;
		flex-direction: column;
		align-items: center;
	}

	.time-counter-display {
		background-color: #ffffff;
		color: #000000;
		border-radius: 5px;
		padding: 10px;
		margin-top: 10px;
	}

	footer h4 {
		margin-bottom: 10px;
	}
</style>
