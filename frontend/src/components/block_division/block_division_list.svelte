<script lang="ts">
	import IconButton, { Icon } from "@smui/icon-button";
	import { mdiPlus, mdiPencil, mdiContentCopy } from "@mdi/js";
	import { DisplayMode } from "../../commons/commons";
	import type {
		BlockDivisionState,
		BlockDivisionStateList
	} from "../../post/results/block_division_state";

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
	<table>
		{#each Object.keys(list) as block_division_id}
			<tr>
				<td>
					{block_division_id}
				</td>
				<td>
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
				</td>
				<td>
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
				</td>
				<td class="table_padding"></td>
			</tr>
		{/each}
	</table>
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
	table {
		table-layout: auto;
	}
	td {
		white-space: nowrap;
	}
	.table_padding {
		width: 100%;
	}
</style>
