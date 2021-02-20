<script lang="typescript">
  import SecondaryButton from "../../components/buttons/Secondary.svelte";
  import ActionRow from "../../components/buttons/ActionRow.svelte";
  import DialogHeader from "../../components/headers/DialogHeader.svelte";
  import InactivePlayer from "../../components/game/InactivePlayer.svelte";
  import Credits from "../../components/game/Credits.svelte";
  import Card from "../../components/game/Card.svelte";
  import Blinds from "../../components/game/Blinds.svelte";
  import type { Game } from "../../types/proto/game";
  import type { Player, OwnPlayer } from "../../types/proto/player";
  import Divider from "../../components/layout/Divider.svelte";

  export let currentGame: Game;
  export let players: Record<string, { player: Player; active: boolean }>;
  export let ws: WebSocket;
  export let ownPlayer: OwnPlayer;
  export let leaveGame: () => Promise<void>;

  const allPlayers = Object.values(players).sort(
    (a, b) => b.player.position - a.player.position
  );
  const nextPlayers = allPlayers.filter(
    (p) => p.player.position > ownPlayer.position
  );
  const prevPlayers = allPlayers.filter(
    (p) => p.player.position < ownPlayer.position
  );
</script>

<DialogHeader>Round</DialogHeader>
<div class="mb-4">
  <div class="grid grid-flow-col gap-4">
    {#each [...nextPlayers, ...prevPlayers] as p}
      <div class="border border-gray-200 p-4 rounded-md">
        <div class="font-bold flex justify-between items-center mb-2">
          {p.player.name}
          <InactivePlayer isActive={p.active} />
          <Blinds game={currentGame} playerId={p.player.id} />
        </div>
        <div><Credits amount={p.player.credits} /></div>
      </div>
    {/each}
  </div>
  <Divider />
  <div class="flex justify-center">
    <div>
      Pot: <Credits amount={currentGame.pot} /> | Blinds: <Credits
        amount={currentGame.blind}
      />
    </div>
  </div>
  <Divider />
  <div class="flex justify-between border border-gray-200 p-4 rounded-md">
    <div>
      <div class="font-bold">{ownPlayer.name}</div>
      <div><Credits amount={ownPlayer.credits} /></div>
    </div>
    <div class="grid gap-4 grid-flow-col">
      {#each ownPlayer.cards as card}
        <Card {card} />
      {/each}
    </div>
    <div>
      <Blinds game={currentGame} playerId={ownPlayer.id} />
    </div>
  </div>
</div>
<ActionRow>
  <div />
  <SecondaryButton onClick={leaveGame}>Leave Game</SecondaryButton>
</ActionRow>
