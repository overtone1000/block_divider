<script lang="ts">
	import IconButton, { Icon } from "@smui/icon-button";
	import { mdiPlus, mdiPencil, mdiContentCopy } from "@mdi/js";
	import { DisplayMode } from "../../commons/commons";

	export let list: BlockDivisionStateList;
	export let selected_division: [string, BlockDivisionState] | undefined = undefined;
	export let set_display_mode: (mode: DisplayMode) => void;

	let add = () => {
		set_display_mode(DisplayMode.Create);
	};

	let to_tuple = (block_division_id: string) => {
		return [block_division_id, list[block_division_id]] as [string, BlockDivisionState];
	};
</script>

<div class="outer">
	<div class="list">
		{#each Object.keys(list) as block_division_id}
			<div class="list_row">
				<div>
					{block_division_id}
				</div>
				<IconButton
					on:click={() => {
						selected_division = to_tuple(block_division_id);
						set_display_mode(DisplayMode.Modify);
						return false;
					}}
				>
					<Icon tag="svg" viewBox="0 0 24 24">
						<path fill="currentColor" d={mdiPencil} />
					</Icon>
				</IconButton>
				<IconButton
					on:click={() => {
						selected_division = to_tuple(block_division_id);
						set_display_mode(DisplayMode.Create);
						return false;
					}}
				>
					<Icon tag="svg" viewBox="0 0 24 24">
						<path fill="currentColor" d={mdiContentCopy} />
					</Icon>
				</IconButton>
			</div>
		{/each}
	</div>
	<IconButton on:click={add}>
		<Icon tag="svg" viewBox="0 0 24 24">
			<path fill="currentColor" d={mdiPlus} />
		</Icon>
	</IconButton>
</div>

<style>
	.outer {
		display: flex;
		flex-direction: column;
	}
	.list {
		display: flex;
		flex-direction: column;
	}
	.list_row {
		display: flex;
		flex-direction: row;
		align-items: center;
	}
</style>
