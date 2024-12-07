<script>
  import { ChevronLeft, ChevronRight, Circle } from "lucide-svelte";
  import Button from "./pagbutton.svelte";
  export let numOfPages = 1;
  export let prevPage;
  export let nextPage;
  /**
   * @type {number}
   */
  export let currentPage;
</script>

<div
  class="absolute p-4 top-[82%] left-1/2 -translate-x-1/2 flex justify-center bg-gradient-radial from-black to-transparent items-center mt-4"
>
  <Button onClick={prevPage}>
    <ChevronLeft size={50} color="#D9D9D9" />
  </Button>

  {#each Array(numOfPages) as _, index}
    {#if index === currentPage - 1}
      <Button onClick={() => {}}>
        <Circle size={50} fill="#D9D9D9" color="#D9D9D9" />
      </Button>
    {:else}
      <Button
        onClick={() => {
          currentPage = index + 1;
        }}
      >
        <Circle size={20} fill="#D9D9D9" color="#D9D9D9" />
      </Button>
    {/if}
  {/each}

  <Button onClick={nextPage}>
    <ChevronRight size={50} color="#D9D9D9" />
  </Button>
</div>

<style>
  .bg-gradient-radial {
    background-image: radial-gradient(
      closest-side ellipse at center,
      #000000,
      transparent
    );
  }
</style>
