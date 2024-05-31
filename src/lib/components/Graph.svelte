<script>
	import { Bar } from 'svelte-chartjs';
	import { Chart, Title, Tooltip, Legend, BarElement, CategoryScale, LinearScale } from 'chart.js';
	export let graphData; // Received as a prop

	$: console.log('Graph data received:', graphData);

	Chart.register(Title, Tooltip, Legend, BarElement, CategoryScale, LinearScale);
</script>

<Bar data={$graphData} options={{ responsive: true }} />
