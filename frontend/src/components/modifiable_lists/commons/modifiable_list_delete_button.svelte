<script lang="ts" generics="T">
	import IconButton, { Icon } from "@smui/icon-button";
	import { mdiDelete } from "@mdi/js";
	export let list: T[];
	export let index: number;
</script>

<IconButton
	on:click={() => {
		list.splice(index, 1);
		list = list;
	}}
>
	<Icon tag="svg" viewBox="0 0 24 24">
		<path fill="currentColor" d={mdiDelete} />
	</Icon>
</IconButton>

<style>
</style>
