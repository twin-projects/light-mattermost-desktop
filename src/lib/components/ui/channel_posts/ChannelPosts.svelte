<script lang="ts">
    import type { PostThread } from "$lib/types/posts.model"

    export let thread: PostThread = null;
</script>

<section data-thread-len={thread?.order.length}>
{#if thread}
    {#each thread.order as postId}
        <ChannelPost post={thread.posts[postId]} />
    {/each}
{/fi}
</section>
