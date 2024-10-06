<script lang="ts">
	import Container from "./container.svelte";
	import { onMount } from "svelte";

	let data: undefined | string = "Loading";

	let get_state = () => {
		fetch("localhost:8181/bdd").then((result) => {
			result.json().then((json) => {
				data = json;
			});
		});
	};

	onMount(async () => {
		get_state();
	});
</script>

<Container title="Block Division">
	<div slot="contents">
		<div>{data}</div>
	</div>
</Container>

<style>
</style>
