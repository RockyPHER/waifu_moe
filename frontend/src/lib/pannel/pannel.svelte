<script>
  // @ts-nocheck

  import PaginationBar from "./paginationBar/paginationBar.svelte";
  export let pannelImage;
  export let numOfPages = 1;
  export let show;
  export let prevPage;
  export let nextPage;
  export let currentPage;
</script>

<div
  class="absolute w-full h-full top-0 left-0 bg-cover"
  style="background-image: url(${pannelImage}); display: {show
    ? 'absolute'
    : 'none'};"
>
  <div>
    <slot />
    <PaginationBar bind:currentPage {numOfPages} {prevPage} {nextPage} />
  </div>
</div>
