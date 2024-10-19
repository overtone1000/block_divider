<script lang="ts">
	import Button, { Label } from "@smui/button";

	export let save_func: () => void;
	export let discard_func: () => void;
</script>

<div class="outer">
	<Button on:click={save_func} variant="raised">
		<Label>Save</Label>
	</Button>
	<Button on:click={discard_func} color="secondary" variant="raised">
		<Label>Discard</Label>
	</Button>
</div>

<style>
	.outer {
		display: flex;
		flex-direction: row;
	}
</style>
