<script>
  // @ts-nocheck

  import { getAnimes, getCharacters } from "./api/fetch";
  import Card from "./lib/card/card.svelte";
  import Layout from "./lib/layout/layout.svelte";
  import Pannel from "./lib/pannel/pannel.svelte";

  let characters;
  let animes;
  let currentPage = 1;

  getCharacters().then((chars) => {
    characters = chars.data;
  });
  getAnimes().then((anim) => {
    animes = anim.data;
  });

  const nextPage = () => {
    currentPage = (currentPage % 3) + 1;
  };
  const prevPage = () => {
    currentPage = ((currentPage - 2 + 3) % 3) + 1;
  };
</script>

<main>
  {#each animes as anime}
    <Layout animeLogo={anime.logo_url}>
      {#each characters as character}
        {#if character.anime_id === anime.id}
          <Pannel
            bind:currentPage
            pannelImage={character.pannel_url}
            numOfPages={3}
            {nextPage}
            {prevPage}
            show={currentPage === character.id ? true : false}
          >
            <Card cardImage={character.card_url} cardName={character.name} />
          </Pannel>
        {/if}
      {/each}
    </Layout>
  {/each}
</main>
