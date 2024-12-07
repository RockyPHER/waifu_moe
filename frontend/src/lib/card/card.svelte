<script>
  // @ts-nocheck
  import { ChevronLeft, ChevronRight, Circle } from "lucide-svelte";
  import { getCharacters } from "../../api/fetch";

  export let cardImage;
  export let cardName;
</script>

<main>
  <div class="absolute left-[12%] top-1/2 -translate-y-1/2">
    <div
      class="w-[320px] h-[400px] flex justify-center items-center shadow-md shadow-black rounded-xl overflow-hidden"
    >
      <img
        class="w-full h-full object-cover"
        src={`${cardImage}`}
        alt={`${cardName}`}
      />
    </div>
  </div>
</main>
