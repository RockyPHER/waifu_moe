<script>
  // @ts-nocheck

  import { ChevronLeft, ChevronRight, Circle } from "lucide-svelte";
  import Button from "./pagbutton.svelte";
  import { getCharacters } from "../../api/fetch";

  let characters;

  getCharacters().then((chars) => {
    console.log(chars.data);
    characters = chars.data;
  });
</script>

<main>
  <div class="absolute right-[12%] top-1/2 -translate-y-1/2">
    {#each characters as character}
      <div
        class="w-[320px] h-[400px] flex justify-center items-center shadow-md shadow-black rounded-xl overflow-hidden"
      >
        <img
          class="w-full h-full object-cover"
          src={`${character.card_url}`}
          alt={`${character.name}`}
        />
      </div>
    {/each}

    <div class="flex justify-center items-center mt-4">
      <Button>
        <ChevronLeft size={30} color="black" />
      </Button>

      {#each characters as page}
        <Button>
          <Circle size={10} fill="black" color="black" />
        </Button>
      {/each}

      <Button>
        <ChevronRight size={30} color="black" />
      </Button>
    </div>
  </div>
</main>
