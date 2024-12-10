<script>
  // @ts-nocheck

  import { Circle } from "lucide-svelte";
  import Button from "./pagbutton.svelte";
  export let numOfPages = 1;
  export let currentPageIdx;

  const handleClick = (idx) => {
    currentPageIdx = idx;
    console.log(currentPageIdx, idx);
  };
</script>

<div
  class="absolute p-4 top-[82%] left-1/2 -translate-x-1/2 flex justify-center items-center bg-gradient-radial from-black to-transparent mt-4"
>
  {#each Array(numOfPages) as _, index}
    {#if index === currentPageIdx}
      <Button onClick={() => handleClick(index)}>
        <Circle size={25} fill="#D9D9D9" color="#D9D9D9" />
      </Button>
    {:else}
      <Button onClick={() => handleClick(index)}>
        <Circle size={15} fill="#D9D9D9" color="#D9D9D9" />
      </Button>
    {/if}
  {/each}
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
