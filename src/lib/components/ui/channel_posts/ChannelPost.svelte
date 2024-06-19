<script lang="ts">
    import type { Post } from "$lib/types/posts.model"

    export let post: Post = null;
</script>

{#if post }
<div data-post-id={ post.id } data-post-type={ post.type }>
{post.message}
</div>
{/if}
