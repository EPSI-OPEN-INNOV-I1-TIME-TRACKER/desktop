<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { onMount } from 'svelte';
	import { ClockSolid, CirclePauseSolid } from 'flowbite-svelte-icons'

	let activeWindowText: string = 'No active window';
	let timeSpent: object = {};

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

	onMount(() => {
		const intervalId = setInterval(fetchActiveWindowInfo, 1000);
		return () => clearInterval(intervalId);
	});
</script>

<div id="app">
	<header>
		<div id="left-section">
			<ClockSolid size="xs" />
		</div>
		<div id="middle-section">
			<h1>TIME'TRACKER</h1>
		</div>
		<div id="right-section">
			<CirclePauseSolid size="xs" />
			<!-- Logo "Paramètres" -->
			<!-- Logo "Utilisateur" -->
		</div>
	</header>

	<main>
		<div id="left-side-menu">
			<ul>
				<li>Visual Studio Code</li>
				<li>Discord</li>
				<li>Spotify</li>
				<li>Warframe</li>
			</ul>
			<div id="manage-btn">
				<p>GERER</p>
			</div>
		</div>

		<div id="tracking">
			<!-- Insérer section graphique pour le tracking -->
		</div>

		<div>
			<!-- Insérer la section avec les applications les plus ouvertes -->
		</div>
	</main>

	<footer>
		<div class="time-tracking">
			<h4>TEMPS PC AUJOURD'HUI :</h4>
			<p>04:06:45</p>
		</div>

		<div class="time-tracking">
			<h4>TEMPS D'APPLICATION SELECTIONNEE :</h4>
			<p>04:06:45</p>
		</div>
	</footer>
</div>

<style>
	* {
		margin: 0;
		padding: 0;
		box-sizing: border-box;
		font-family: Arial, sans-serif;
	}

	body,
	html {
		height: 100%;
		background-color: #eee6d8;
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

	header #left-section,
	header #middle-section,
	header #right-section {
		display: flex;
		align-items: center;
	}

	header h1 {
		margin: 0;
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
		background-color: #ffffff;
		margin: 20px;
		border-radius: 8px;
		border: 2px solid #d8d8d8;
	}

	footer {
		display: flex;
		justify-content: space-around;
		padding: 30px 60px;
		border-radius: 10px 10px 10px 10px;
		background-color: #804000;
		color: white;
	}

	footer .time-tracking {
		display: flex;
		flex-direction: column;
		align-items: center;
	}

	footer h4 {
		margin-bottom: 10px;
	}
</style>
