<script>
  import Button from "./colButton.svelte";
  import { Circle } from "lucide-svelte";
</script>

<div class="absolute p-3 flex flex-col gap-2 top-1/2 -translate-y-1/2 right-0">
  <Button>
    <Circle size={50} fill="#D9D9D9" color="#D9D9D9" />
  </Button>
  <div class="w-[2px] h-[80px] bg-[#D9D9D9]"></div>
  <Button>
    <Circle size={50} fill="#D9D9D9" color="#D9D9D9" />
  </Button>
  <div class="w-[2px] h-[80px] bg-[#D9D9D9]"></div>
  <Button>
    <Circle size={50} fill="#D9D9D9" color="#D9D9D9" />
  </Button>
</div>
