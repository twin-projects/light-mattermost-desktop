<script lang="ts">
    import type { PostThread } from "$lib/types/posts.model"
	import ChannelPost from '$lib/components/ui/channel_posts/ChannelPost.svelte';

    export let thread: PostThread = null;
</script>

<section data-thread-len={thread?.order.length} class="">
{#if thread}
    {#each thread.order.reverse() as postId}
        <ChannelPost post={thread.posts[postId]} />
    {/each}
{:else}
<div>No messages</div>
{/if}
</section>
