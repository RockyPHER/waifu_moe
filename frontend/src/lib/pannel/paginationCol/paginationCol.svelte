<script>
  // @ts-nocheck

  import Button from "./colButton.svelte";
  import { Circle } from "lucide-svelte";
  /**
   * @type {number}
   */
  export let currentAnimeIdx;

  const handleClick = (idx) => {
    currentAnimeIdx = idx;
    console.log(currentAnimeIdx, idx);
  };
</script>

<div
  class="absolute p-3 flex flex-col justify-center items-center gap-2 top-1/2 -translate-y-1/2 right-0"
>
  {#each Array(3) as _, index}
    {#if index === currentAnimeIdx}
      <Button onClick={() => handleClick(index)}>
        <Circle size={20} fill="#D9D9D9" color="#D9D9D9" />
      </Button>
    {:else}
      <Button onClick={() => handleClick(index)}>
        <Circle size={10} fill="#D9D9D9" color="#D9D9D9" />
      </Button>
    {/if}
    {#if index !== 2}
      <div class="w-[4px] h-[80px] bg-[#D9D9D9] rounded"></div>
    {/if}
  {/each}
</div>
