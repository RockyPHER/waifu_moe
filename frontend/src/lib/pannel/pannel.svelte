<script>
  // @ts-nocheck

  import PaginationBar from "./paginationBar/paginationBar.svelte";
  import PagintaionCol from "./paginationCol/paginationCol.svelte";
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
  <div class="absolute z-10 w-full h-full top-0 left-0 bg-gradient-radial">
    <slot />
    <PaginationBar bind:currentPage {numOfPages} {prevPage} {nextPage} />
    <PaginationCol />
  </div>
</div>

<style>
  .bg-gradient-radial {
    background: radial-gradient(
      farthest-corner ellipse at center,
      transparent,
      #000000
    );
  }
</style>
