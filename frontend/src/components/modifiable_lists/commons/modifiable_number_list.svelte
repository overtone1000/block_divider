<script lang="ts">
	import IconButton, { Icon } from "@smui/icon-button";
	import { mdiDelete } from "@mdi/js";
	import Textfield from "@smui/textfield";
	import ModifiableListContainer from "./modifiable_list_container.svelte";
	import ModifiableListDeleteButton from "./modifiable_list_delete_button.svelte";
	export let list: number[];
	export let label: string;
</script>

<ModifiableListContainer
	add={() => {
		list.push(0);
		list = list;
	}}
>
	{#each list as item, index}
		<div class="area">
			<Textfield {label} bind:value={item} />
			<ModifiableListDeleteButton bind:list {index} />
		</div>
	{/each}
</ModifiableListContainer>

<style>
	.area {
		margin: 10px;
		padding: 10px;
		border-width: 1px;
		border-style: solid;
		border-radius: 10px;
	}
</style>
