<script>
  import { ChevronLeft, ChevronRight, Circle } from "lucide-svelte";
  import Button from "./pagbutton.svelte";
  export let pannelImage;
  export let numOfPages = 1;
  export let show;
  export let prevPage;
  export let nextPage;
</script>

<div
  class="absolute w-full h-full top-0 left-0 bg-cover"
  style="background-image: url(${pannelImage}); display: {show
    ? 'absolute'
    : 'none'};"
>
  <div>
    <slot />
    <div
      class="absolute top-[82%] right-[16%] flex justify-center items-center mt-4"
    >
      <Button onClick={prevPage}>
        <ChevronLeft size={30} color="black" />
      </Button>

      {#each Array(numOfPages) as _, index}
        <Button onClick={() => {}}>
          <Circle size={10} fill="black" color="black" />
        </Button>
      {/each}

      <Button onClick={nextPage}>
        <ChevronRight size={30} color="black" />
      </Button>
    </div>
  </div>
</div>
