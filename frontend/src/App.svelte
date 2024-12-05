<script>
  import { getAnimes, getCharacters } from "./api/fetch";
  import Card from "./lib/card/card.svelte";
  import Pannel from "./lib/pannel/pannel.svelte";

  let characters;
  let animes;

  getCharacters().then((chars) => {
    characters = chars.data;
  });
  getAnimes().then((anim) => {
    animes = anim.data;
  });
</script>

<main>
  <div class="w-screen h-screen flex justify-center items-center">
    <Pannel>
      <Card />
    </Pannel>
  </div>
</main>
