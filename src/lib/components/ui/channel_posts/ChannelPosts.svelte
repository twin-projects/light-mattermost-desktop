<script lang="ts">
    import type { PostThread } from "$lib/types/posts.model"

    export let thread: PostThread = null;
</script>

<section data-thread-len={thread?.posts.length}>
</section>
