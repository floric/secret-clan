<script lang="typescript">
  import { Game } from "../../types/proto/game";

  export let game: Game;
  export let playerId: string;
</script>

{#if game.smallBlindId == playerId}
  <div
    class="rounded-full text-xs bg-green-700 text-white w-6 h-6 justify-center items-center inline-flex"
  >
    B
  </div>
{:else if game.bigBlindId == playerId}
  <div
    class="rounded-full text-xs bg-red-800 text-white w-6 h-6 justify-center items-center inline-flex"
  >
    S
  </div>
{/if}
