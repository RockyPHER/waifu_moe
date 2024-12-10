<script>
  import { Loader } from "lucide-svelte";
  import { getAnimes, getCharacters, getCharactersByAnime } from "./api/fetch";
  import Card from "./lib/card/card.svelte";
  import Layout from "./lib/layout/layout.svelte";
  import Pannel from "./lib/pannel/pannel.svelte";
  import PaginationBar from "./lib/components/paginationBar/main.svelte";
  import PaginationCol from "./lib/components/paginationCol/main.svelte";

  let animes = [];
  let NumAnimes = 0;
  let characters = [];
  let isLoading = true;
  let currentPageIdx = 0; // Index for currently displayed character
  let currentAnimeIdx = 0; // Index for currently displayed anime
  let charactersArray = {};
  let animeCharacterCount = {}; // Object to store character count per anime

  // Fetch data for animes and characters
  async function fetchData() {
    try {
      const [animeRes, charRes] = await Promise.all([
        getAnimes(),
        getCharacters(),
      ]);

      animes = animeRes.data;
      characters = charRes.data;

      // Set defaults
      if (animes.length > 0) {
        currentAnimeIdx = 0; // First anime by index
        NumAnimes = animes.length;
      }
      if (characters.length > 0) {
        // Count characters per anime
        characters.forEach((character) => {
          if (!animeCharacterCount[character.anime_id]) {
            animeCharacterCount[character.anime_id] = 0;
            charactersArray[character.anime_id] = [];
          }
          animeCharacterCount[character.anime_id] += 1;
          charactersArray[character.anime_id].push(character);
        });
      }
    } catch (err) {
      console.error("Error fetching data:", err);
    } finally {
      isLoading = false;
    }
  }

  fetchData().then(() => console.log(charactersArray));
</script>

<main class="w-screen h-screen overflow-hidden">
  {#if isLoading}
    <div class="w-screen h-screen bg-gray-300 flex justify-center items-center">
      <div
        class="flex flex-col justify-center items-center bg-gray-400 rounded p-10"
      >
        <Loader size={50} color="black" />
        <p class="text-center text-white text-2xl font-bold">Loading...</p>
      </div>
    </div>
  {:else if animes.length > 0 && characters.length > 0}
    <div
      style="height: {NumAnimes *
        100}vh; transform: translateY(-{currentAnimeIdx * 100}vh);"
      class="relative flex flex-col transition-all"
    >
      <PaginationCol bind:currentAnimeIdx />
      <PaginationBar bind:currentPageIdx numOfPages={NumAnimes} />
      {#each animes as anime}
        <Layout animeLogo={anime.logo_url} animeID={anime.id}>
          <div
            class="relative h-screen flex"
            style="transform: translateX(-{currentPageIdx * 100}vw);
            width: {charactersArray[anime.id].length * 100}vw"
          >
            {#each charactersArray[anime.id] as character, index}
              <Pannel
                pannelImage={character.pannel_url}
                numOfPages={animeCharacterCount[anime.id]}
              >
                <Card
                  cardImage={character.card_url}
                  cardName={character.name}
                />
              </Pannel>
            {/each}
          </div>
        </Layout>
      {/each}
    </div>
  {:else}
    <div class="w-screen h-screen bg-gray-300 flex justify-center items-center">
      <p
        class="text-center text-white text-2xl font-bold bg-red-500 p-2 rounded"
      >
        No data available.
      </p>
    </div>
  {/if}
</main>
