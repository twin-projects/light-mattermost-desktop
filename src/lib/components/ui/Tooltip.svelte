<script lang="ts">
	export let title = '';
	export let fontSize = 'text-xs';

	let isHovered = false;
	let x: number;
	let y: number;

	const mouseOver = (event: MouseEvent) => {
		isHovered = true;
		x = event.pageX + 5;
		y = event.pageY + 5;
	};
	const mouseMove = (event: MouseEvent) => {
		x = event.pageX + 5;
		y = event.pageY + 5;
	};
	const mouseLeave = () => {
		isHovered = false;
	};
</script>

<div
	role="button"
	tabindex="0"
	on:focus={() => 0}
	on:mouseover={mouseOver}
	on:mouseleave={mouseLeave}
	on:mousemove={mouseMove}>
	<slot />
</div>

{#if isHovered}
	<div
		style="top: {y}px; left: {x}px;"
		class={`tooltip w-36 text-black ${fontSize}`}>
		{title}
	</div>
{/if}

<style lang="css">
    .tooltip {
        border: 1px solid #ddd;
        box-shadow: 1px 1px 1px #ddd;
        background: white;
        border-radius: 4px;
        padding: 4px;
        position: absolute;
    }
</style>