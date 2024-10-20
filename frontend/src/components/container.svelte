<script lang="ts">
	import { mdiThemeLightDark } from "@mdi/js";
	import IconButton, { Icon } from "@smui/icon-button";

	export let title: string;

	const light_css = "/smui.css";
	const dark_css = "/smui-dark.css";

	let dark_mode: boolean | undefined = undefined;
</script>

<svelte:head>
	{#if dark_mode === undefined}
		<link rel="stylesheet" href={light_css} media="(prefers-color-scheme: light)" />
		<link rel="stylesheet" href={dark_css} media="screen and (prefers-color-scheme: dark)" />
	{:else if dark_mode}
		<link rel="stylesheet" href={light_css} media="print" />
		<link rel="stylesheet" href={dark_css} media="screen" />
	{:else}
		<link rel="stylesheet" href={light_css} />
	{/if}
</svelte:head>

<div class="vp_fill">
	<div class="top_menu">
		<div class="top_menu_item">{title}</div>
		<div class="spacer"></div>
		<div class="top_menu_item">
			<IconButton on:click={() => (dark_mode = !dark_mode)} toggle pressed={dark_mode}>
				<Icon tag="svg" viewBox="0 0 24 24" on>
					<path fill="currentColor" d={mdiThemeLightDark} />
				</Icon>
				<Icon tag="svg" viewBox="0 0 24 24">
					<path fill="currentColor" d={mdiThemeLightDark} />
				</Icon>
			</IconButton>
		</div>
	</div>
	<div class="page">
		<slot name="contents">Nothing in slot 1</slot>
	</div>
</div>

<style>
	.vp_fill {
		width: 100vw;
		height: 100vh;
		max-width: 100vw;
		max-height: 100vh;
		overflow: hidden;
		display: flex;
		flex-direction: column;
	}
	.top_menu {
		display: flex;
		flex-direction: row;
	}
	.top_menu_item {
		display: flex;
		align-items: center;
	}
	.page {
		display: flex;
		flex-direction: column;
		flex-grow: 1;
		min-height: 1px;
		overflow: auto;
	}
	.spacer {
		flex-grow: 1;
	}
</style>
