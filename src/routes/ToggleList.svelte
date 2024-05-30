<script lang="ts">
	export let apps: string[] = [];
	export let activeApp: string;
	export let onToggle: (app: string) => void;

	function handleToggle(app: string) {
		onToggle(app);
	}
</script>

<ul>
	{#each apps as app}
		<li class:active={activeApp === app}>
			<button on:click={() => handleToggle(app)}>
				{app}
			</button>
		</li>
	{/each}
</ul>

<style>
	ul {
		list-style: none;
		padding: 0;
	}

	li {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 10px;
		color: #ffffff;
		border-radius: 5px;
		transition: background-color 0.3s;
	}

	li.active {
		background-color: #d18b47;
		font-weight: bold;
	}

	li:hover {
		background-color: #d18b47;
		font-weight: bold;
	}

	button {
		background: none;
		border: none;
		color: inherit;
		font: inherit;
		cursor: pointer;
		padding: 0;
	}
</style>
