<script>
  export let animeLogo;
  export let animeName;
  export let showAnime;
</script>

<div class="overflow-hidden" style="display: {showAnime ? 'block' : 'none'}">
  <div
    class="absolute z-20 h-16 w-full top-0 left-0 flex items-center justify-start p-10 bg-gradient-to-b from-black to-transparent bg-opacity-40"
  >
    <p class="text-2xl font-bold text-white">{animeName}</p>
    <img class="w-16 h-16" src={`${animeLogo}`} alt="logo" />
  </div>
  <slot />
</div>
