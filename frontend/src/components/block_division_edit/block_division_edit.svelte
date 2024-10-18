<script lang="ts">
	import Textfield from "@smui/textfield";
	import Select, { Option } from "@smui/select";

	export let block_division_tuple: [string, BlockDivisionState];

	let block_division = block_division_tuple[1];

	//Don't need to manage bucket states. That's internal to the program.
	//Don't need to manage selections. That's controlled by users.

	/*Need to be able to change:
		[x]	block_division.basis.label;
		[ ]	block_division.basis.bucket_definitions;
		[ ]	block_division.basis.participant_definitions;
		[ ] block_division.basis.selection_round_names;
		[x]	block_division.current_open_round;
	*/

	let round_keys = Object.keys(block_division.basis.selection_round_names) as unknown as number[];
</script>

<div>
	<Textfield label="Label" bind:value={block_division.basis.label} />

	<Select bind:value={block_division.current_open_round} label="Current Open Round">
		{#each round_keys as round_key}
			<Option value={round_key}>{block_division.basis.selection_round_names[round_key]}</Option>
		{/each}
	</Select>
</div>

<style>
</style>
