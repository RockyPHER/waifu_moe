<script>
  // @ts-nocheck

  import PaginationBar from "./paginationBar/paginationBar.svelte";
  import PaginationCol from "./paginationCol/paginationCol.svelte";

  export let pannelImage;
  export let showChar; // Boolean to control visibility
  export let currentAnimeIdx;
  export let currentPageIdx;
  export let numOfPages = 1;
</script>

{#if showChar}
  <div
    class="absolute w-full h-full top-0 left-0 bg-cover"
    style="background-image: url(${pannelImage});"
  >
    <div class="absolute z-10 w-full h-full top-0 left-0 bg-gradient-radial">
      <slot />
      <PaginationBar bind:currentPageIdx {numOfPages} />
      <PaginationCol bind:currentAnimeIdx />
    </div>
  </div>
{/if}

<style>
  .bg-gradient-radial {
    background: radial-gradient(
      farthest-corner ellipse at center,
      transparent,
      #000000
    );
  }
</style>
