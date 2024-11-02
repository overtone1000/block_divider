<script lang="ts">
	import IconButton, { Icon } from "@smui/icon-button";
	import { mdiPlus } from "@mdi/js";
	export let add: () => void;
</script>

<div>
	<slot />
	<IconButton on:click={add}>
		<Icon tag="svg" viewBox="0 0 24 24">
			<path fill="currentColor" d={mdiPlus} />
		</Icon>
	</IconButton>
</div>

<style>
</style>
