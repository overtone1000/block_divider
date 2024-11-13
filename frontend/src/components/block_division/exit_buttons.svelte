<script lang="ts">
	import Dialog, { Title, Content, Actions } from "@smui/dialog";
	import Button, { Label } from "@smui/button";

	export let save_func: (() => void) | undefined = undefined;
	export let discard_func: (() => void) | undefined = undefined;
	export let delete_func: (() => void) | undefined = undefined;

	let confirmation_dialog_open: boolean = false;
</script>

<div class="outer">
	<Dialog bind:open={confirmation_dialog_open} aria-labelledby="title" aria-describedby="content">
		<!-- Title cannot contain leading whitespace due to mdc-typography-baseline-top() -->
		<Title id="title">Deletion</Title>
		<Content id="content">Are you sure you want to delete this?</Content>
		<Actions>
			<Button
				on:click={() => {
					confirmation_dialog_open = false;
				}}
			>
				<Label>No</Label>
			</Button>
			<Button
				on:click={() => {
					if (delete_func !== undefined) {
						delete_func();
					}

					confirmation_dialog_open = false;
				}}
			>
				<Label>Yes</Label>
			</Button>
		</Actions>
	</Dialog>

	{#if save_func !== undefined}
		<div class="button">
			<Button on:click={save_func} variant="raised">
				<Label>Save</Label>
			</Button>
		</div>
	{/if}
	{#if discard_func !== undefined}
		<div class="button">
			<Button on:click={discard_func} color="secondary" variant="raised">
				<Label>Discard</Label>
			</Button>
		</div>
	{/if}
	{#if delete_func !== undefined}
		<div class="button">
			<Button on:click={() => (confirmation_dialog_open = true)} color="secondary" variant="raised">
				<Label>Delete</Label>
			</Button>
		</div>
	{/if}
</div>

<style>
	.outer {
		display: flex;
		flex-direction: row;
		align-items: flex-start;
		padding: 10px;
	}
	.button {
		padding: 10px;
	}
</style>
