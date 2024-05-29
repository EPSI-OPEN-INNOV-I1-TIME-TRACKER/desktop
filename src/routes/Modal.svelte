<script lang="ts">
	import { createEventDispatcher } from 'svelte';

	export let modalTitle: string;

	const dispatch = createEventDispatcher();

	function close() {
		dispatch('close');
	}
</script>

<button class="modal-backdrop" on:click|self={close}>
	<div class="modal">
		<h2>{modalTitle}</h2>
		<div class="modal-content">
			<slot></slot>
		</div>
		<button on:click={close}>FERMER</button>
	</div>
</button>

<style>
	.modal-backdrop {
		position: fixed;
		top: 0;
		left: 0;
		width: 100vw;
		height: 100vh;
		background-color: rgba(0, 0, 0, 0.5);
		display: flex;
		justify-content: center;
		align-items: center;
	}

	.modal {
		background-color: white;
		padding: 20px;
		border-radius: 10px;
		max-width: 500px;
		width: 100%;
	}

	.modal-content {
		display: flex;
		flex-direction: column;
		align-items: center; /* Center align items */
		margin-bottom: 20px;
	}

	button {
		align-self: center; /* Center align button */
		background-color: antiquewhite;
		border: none;
		padding: 10px;
		border-radius: 5px;
		cursor: pointer;
		text-align: center;
		font-weight: bold;
	}
</style>
