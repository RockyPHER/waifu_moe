<script>
  export let pannelImage;
</script>

<div
  class="relative w-full h-full bg-cover"
  style="background-image: url(${pannelImage});"
>
  <div class="absolute z-10 w-full h-full top-0 left-0 bg-gradient-radial">
    <slot />
  </div>
</div>

<style>
  .bg-gradient-radial {
    background: radial-gradient(
      farthest-corner ellipse at center,
      transparent,
      #000000
    );
  }
</style>
