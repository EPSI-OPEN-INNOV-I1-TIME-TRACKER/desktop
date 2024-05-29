<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { onMount } from 'svelte';
	import Icon from '@iconify/svelte';
	import Modal from './Modal.svelte';
	import Switch from './Switch.svelte'

	let apps = [
		'Visual Studio Code',
		'Discord',
		'Spotify',
		'Warframe',
		'Google Chrome',
		'Microsoft Edge',
		'Steam',
		'Epic Games',
		'Valorant',
		'League of Legends'
	];

	let timeSpentToday: string = '00:00:00';
	let timeSpentSelectedApp: string = '00:00:00';
	let connectedUser: string = 'John Doe';
	let sliderValue: any;

	let activeWindowText: string = 'No active window';
	let timeSpent: object = {};

	let showModal = false;
	let modalTitle: string = ''; // Define the modalTitle variable
	let modalType: string = ''; // Define the modalType variable

	interface ActiveWindow {
		app_name: string;
	}

	async function fetchActiveWindowInfo(): Promise<void> {
		try {
			const activeWindow: ActiveWindow = await invoke('get_active_window_info');
			activeWindowText = activeWindow.app_name;

			const time: number = await invoke('get_current_window_time');
			timeSpent = formatDuration(time);
		} catch (error) {
			console.error('Error fetching active window info:', error);
		}
	}

	function formatDuration(duration: number): any {
		// Convert duration in milliseconds to a readable format
		// Implement this function based on how you want to format the duration
		return duration;
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

	onMount(() => {
		const intervalId = setInterval(fetchActiveWindowInfo, 1000);
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
			<ul>
				{#each apps as app}
					<li>{app}</li>
				{/each}
			</ul>
			<button id="manage-btn">
				<p>GERER</p>
			</button>
		</div>

		<div id="tracking">
			<!-- Insérer section graphique pour le tracking d'utilisation par application -->
		</div>

		<div>
			<!-- Insérer la section avec les applications les plus ouvertes (podium) -->
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
					<Switch bind:value={sliderValue} label="Activer les notifications" fontSize={14} design="slider" />
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

	#left-side-menu ul {
		list-style: none;
	}

	#left-side-menu li {
		padding: 10px;
		color: #ffffff;
		border-radius: 5px;
		transition: background-color 0.3s;
		cursor: pointer;
	}

	#left-side-menu li:hover {
		background-color: #d18b47;
		font-weight: bold;
	}

	#manage-btn {
		background-color: antiquewhite;
		border: none;
		padding: 10px;
		border-radius: 5px;
		cursor: pointer;
		text-align: center;
		font-weight: bold;
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
