<script lang="ts">
	import Textfield from "@smui/textfield";

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
</script>

<div class="columns margins" style="justify-content: flex-start;">
	<Textfield label="Label" bind:value={block_division.basis.label} />
</div>

<style>
</style>
