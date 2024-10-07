<script lang="ts">
	import Container from "./container.svelte";
	import { onMount } from "svelte";

	let data: undefined | string = "Loading";

	let get_state = () => {
		/*
		Test URL
		http://localhost:5173/block_division?user_id=test_user&division_id=test_division_id
		*/
		const urlParams = new URLSearchParams(window.location.search);
		console.debug(urlParams);
		let user_id = urlParams.get("user_id");
		let division_id = urlParams.get("division_id");

		fetch("http://localhost:8181/block_division_state", {
			method: "POST",
			body: JSON.stringify({
				user_id: user_id,
				division_id: division_id
			})
		}).then((result) => {
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
