<script>
  import { ChevronLeft, ChevronRight, Circle } from "lucide-svelte";
  import { getCharacters } from "../../api/fetch";
  import backCard from "./backcard.png";
  export let cardName;
  export let cardImage;
</script>

<main>
  <div class="absolute left-[12%] top-1/2 -translate-y-1/2 flip-box">
    <div class="flip-box-inner">
      <div class="flip-box-front">
        <img
          class="w-full h-full object-cover rounded-[10px]"
          src={`${cardImage}`}
          alt={`${cardName}`}
        />
      </div>
      <div class="flip-box-back">
        <img src={`${backCard}`} alt={`${cardName}`} />
      </div>
    </div>
  </div>
</main>

<style>
  /* The flip box container - set the width and height to whatever you want. We have added the border property to demonstrate that the flip itself goes out of the box on hover (remove perspective if you don't want the 3D effect */
  .flip-box {
    background-color: transparent;
    width: 300px;
    height: 400px;
    perspective: 1000px;
    border-radius: 10px;
  }

  /* This container is needed to position the front and back side */
  .flip-box-inner {
    position: relative;
    width: 100%;
    height: 100%;
    text-align: center;
    transition: transform 0.8s;
    transform-style: preserve-3d;
    border-radius: 10px;
  }

  /* Do an horizontal flip when you move the mouse over the flip box container */
  .flip-box:hover .flip-box-inner {
    transform: rotateY(180deg);
  }

  /* Position the front and back side */
  .flip-box-front,
  .flip-box-back {
    position: absolute;
    width: 100%;
    height: 100%;
    -webkit-backface-visibility: hidden; /* Safari */
    backface-visibility: hidden;
    border-radius: 10px;
  }

  /* Style the front side */
  .flip-box-front {
    background-color: #bbb;
    color: black;
    border-radius: 10px;
  }

  /* Style the back side */
  .flip-box-back {
    color: white;
    transform: rotateY(180deg);
    border-radius: 10px;
  }
</style>
