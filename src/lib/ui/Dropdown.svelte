<script lang="ts">
	import type { PopupSettings } from '@skeletonlabs/skeleton';
	import { ListBox, popup, storePopup } from '@skeletonlabs/skeleton';
	import { arrow, autoUpdate, computePosition, flip, offset, shift } from '@floating-ui/dom';

	storePopup.set({ computePosition, autoUpdate, offset, shift, flip, arrow });

	export let value: string;

	const popupCombobox: PopupSettings = {
		event: 'click',
		target: 'popupCombobox',
		placement: 'bottom',
		closeQuery: '.listbox-item'
	};
</script>

<button class="btn w-36 justify-between" use:popup={popupCombobox}>
	<slot class="capitalize" name="title" />
	<span class="capitalize">{value ?? 'Select'}</span>
</button>

<div class="card w-48 shadow-xl py-2" data-popup="popupCombobox">
	<ListBox>
		<slot name="elements" />
	</ListBox>
	<div class="arrow bg-surface-100-800-token" />
</div>
