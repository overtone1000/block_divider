<script lang="ts">
	import Textfield from "@smui/textfield";
	import ModifiableStringList from "./modifiable_string_list.svelte";
	import ModifiableListContainer from "./modifiable_list_container.svelte";
	import ModifiableListDeleteButton from "./modifiable_list_delete_button.svelte";
	export let bucket_definitions: BucketDefinition[];

	console.debug(bucket_definitions);
</script>

<div class="area">
	<ModifiableListContainer
		add={() => {
			bucket_definitions.push({
				name: "New Bucket",
				available_slots: 0,
				available_ancillaries: []
			});
			bucket_definitions = bucket_definitions;
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
</div>

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
