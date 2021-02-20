<script lang="typescript">
  import { Card, Card_Color } from "../../types/proto/card";

  export let card: Card;

  const isRed =
    card.color === Card_Color.HEARTS || card.color === Card_Color.TILES;
</script>

<div
  class="grid w-12 h-16 border border-gray-700 rounded-md items-center justify-center"
>
  <div class={isRed ? "text-red-600 font-bold" : "text-gray-900 font-bold"}>
    {#if card.color === Card_Color.PIKES}
      <span>♠</span>
    {:else if card.color === Card_Color.HEARTS}
      ♡
    {:else if card.color === Card_Color.TILES}
      ♢
    {:else if card.color === Card_Color.CLOVERS}
      ♣
    {/if}
    {#if card.value === 1}
      <span>A</span>
    {:else if card.value >= 2 && card.value <= 10}
      <span>{card.value}</span>
    {:else if card.value === 11}
      <span>B</span>
    {:else if card.value === 12}
      <span>D</span>
    {:else if card.value === 13}
      <span>K</span>
    {/if}
  </div>
</div>
