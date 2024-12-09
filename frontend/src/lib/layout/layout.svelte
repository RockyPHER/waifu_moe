<script>
  import PaginationCol from "./paginationCol/paginationCol.svelte";
  import { fade } from "svelte/transition";

  export let animeID;
  export let showAnime;
  export let animeLogo;
  export let currentAnimeIdx;

  let width;
  let height;

  switch (animeID) {
    case 1:
      width = "176px";
      height = "99px";
      break;
    case 2:
      width = "220px";
      height = "50px";
      break;
    case 3:
      width = "200px";
      height = "40px";
      break;
    default:
      width = "176px";
      height = "99px";
      break;
  }
</script>

<div
  class="overflow-hidden w-screen h-screen"
  style="display: {showAnime ? 'block' : 'none'}"
>
  <PaginationCol bind:currentAnimeIdx />
  <div
    class="absolute z-20 h-16 w-full top-0 left-0 flex items-center justify-start p-10 bg-black backdrop-blur bg-opacity-40"
  >
    <img
      style="width: {width}; height: {height};"
      src={`${animeLogo}`}
      alt="logo"
    />
  </div>
  <slot />
</div>
