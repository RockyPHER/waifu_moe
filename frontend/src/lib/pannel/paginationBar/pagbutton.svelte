<script>
  export let onClick;
</script>

<button
  on:click={onClick}
  class="w-[40px] h-[40px] p-1 flex justify-center items-center hover:scale-110 active:scale-100 rounded-full"
>
  <slot />
</button>
