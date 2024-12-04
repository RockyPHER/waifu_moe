<script>
  import { ChevronLeft, ChevronRight, Circle } from "lucide-svelte";
  import Button from "./pagbutton.svelte";
  import denjiImage from "../../assets/characters/denjicard.jpg";
  import { getCharacters } from "../../api/anime";

  let characters = [];
  /**
   * @type {any[]}
   */
  let pages = [];

  console.log(getCharacters());

  getCharacters().then((chars) => {
    characters = chars;
    pages = Array.from({ length: characters.length }, (_, index) => index + 1);
  });
</script>

<main>
  <div class="absolute right-[12%] top-1/2 -translate-y-1/2">
    <div
      class="w-[320px] h-[400px] flex justify-center items-center shadow-md shadow-black rounded-xl overflow-hidden"
    >
      <img class="w-full h-full object-cover" src={denjiImage} alt="Denji" />
    </div>
    <div class="flex justify-center items-center mt-4">
      <Button>
        <ChevronLeft size={30} color="black" />
      </Button>

      {#each pages as page}
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
