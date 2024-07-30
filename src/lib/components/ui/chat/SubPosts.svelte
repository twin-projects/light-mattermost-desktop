<script lang="ts">
	import type { PostCollectionModel } from '$lib/types/ui/posts/post.collection.model';
	import { Icon } from 'svelte-icons-pack';
	import { BsReplyAllFill, BsChevronBarUp } from 'svelte-icons-pack/bs';

	export let post: PostCollectionModel;

	let open = false;

</script>

{#if post.children && post.children.length}
	{#if !open}
		<div class="flex justify-end mb-2">
			<div class="ml-2 py-3 px-4 rounded-tl-3xl rounded-tr-xl text-black rounded-br-3xl">
				<button class="flex flex-row" on:click={() => open = !open}>
					<Icon src={BsReplyAllFill} size="12" />
					<span class="pl-1">{post.children.length} replies</span>
				</button>
			</div>

		</div>
	{:else}
		<div class="flex mb-4">
			<div class="w-full hover:bg-amber-100">
				{#each post.children as child}
					<div class="flex justify-end mb-2 w-full">
						<div
							class="ml-2 py-3 px-4 rounded-tr-xl rounded-bl-3xl text-white bg-gray-400 hover:bg-green-300 rounded-3xl">
							{child.message}
						</div>
					</div>
				{/each}
			</div>
			<button
				class="bg-gray-50 hover:bg-gray-200 justify-center animate-bounce w-6 h-full"
				on:click={() => open = !open}
			>
				<Icon src={BsChevronBarUp}
							size="24"
							color="gray" />
			</button>
		</div>
	{/if}
{/if}
