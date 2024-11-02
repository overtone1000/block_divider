<script lang="ts">
	import Textfield from "@smui/textfield";
	import ModifiableStringList from "./commons/modifiable_string_list.svelte";
	import ModifiableListDeleteButton from "./commons/modifiable_list_delete_button.svelte";
	import type { Basis, BucketDefinition } from "../../post/results/state_components/basis";
	import ModifiableListContainer from "./commons/modifiable_list_container.svelte";
	export let basis: Basis;

	let bucket_definitions: BucketDefinition[];
	$: {
		bucket_definitions = basis.bucket_definitions;
	}
</script>

<ModifiableListContainer
	add={() => {
		bucket_definitions.push({
			name: "New Bucket",
			available_slots: 0,
			available_ancillaries: []
		});
		basis = basis;
	}}
>
	{#each bucket_definitions as bucket_definition, index}
		<div class="area">
			<div class="row">
				<Textfield label="Bucket Name" bind:value={bucket_definition.name} />
				<Textfield
					label="Available Slots"
					type="number"
					bind:value={bucket_definition.available_slots}
				/>
				<div class="spacer" />
				<ModifiableListDeleteButton bind:list={bucket_definitions} {index} />
			</div>
			<div class="area">
				Ancillaries
				<ModifiableStringList
					bind:list={bucket_definition.available_ancillaries}
					label="Ancillary Name"
				/>
			</div>
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
	.row {
		display: flex;
		flex-direction: row;
	}
	.spacer {
		flex-grow: 1;
	}
</style>
