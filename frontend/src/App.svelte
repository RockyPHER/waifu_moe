<script>
  // @ts-nocheck

  import { getAnimes, getCharacters, getCharactersByAnime } from "./api/fetch";
  import Card from "./lib/card/card.svelte";
  import Layout from "./lib/layout/layout.svelte";
  import Pannel from "./lib/pannel/pannel.svelte";

  let characters = [];
  let animes = [];
  let currentAnimeIdx = 0; // Index for currently displayed anime
  let currentPageIdx = 0; // Index for currently displayed character
  let NumAnimes = 0;
  let animeCharacterCount = {}; // Object to store character count per anime
  let isLoading = true;
  let charactersArray = {};

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

<main>
  {#if isLoading}
    <div
      class="w-screen h-screen bg-gray-400 flex justify-center items-center text-3xl font-bold"
    >
      Loading...
    </div>
  {:else if animes.length > 0 && characters.length > 0}
    {#each animes as anime}
      <Layout
        animeLogo={anime.logo_url}
        animeName={anime.name}
        showAnime={currentAnimeIdx === anime.id - 1}
      >
        {#each charactersArray[anime.id] as character, index}
          <Pannel
            bind:currentAnimeIdx
            bind:currentPageIdx
            pannelImage={character.pannel_url}
            numOfPages={animeCharacterCount[anime.id]}
            showChar={currentPageIdx === index}
          >
            <Card cardImage={character.card_url} cardName={character.name} />
          </Pannel>
        {/each}
      </Layout>
    {/each}
  {:else}
    <p>No data available.</p>
  {/if}
</main>
