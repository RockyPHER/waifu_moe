<script>
  export let animeLogo;
</script>

<div class=" overflow-hidden">
  <div
    class="absolute z-20 h-16 w-full top-0 left-0 flex items-center justify-start p-10 bg-gradient-to-b from-black to-transparent bg-opacity-40"
  >
    <img class="w-16 h-16" src={`${animeLogo}`} alt="logo" />
  </div>
  <slot />
</div>
