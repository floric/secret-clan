<script lang="typescript">
  import { link, push } from "svelte-spa-router";
  import type { Game } from "../types/Game";
  import PrimaryButton from "../components/buttons/Primary.svelte";
  import SecondaryButton from "../components/buttons/Secondary.svelte";
  import Dialog from "../components/layout/Dialog.svelte";
  import Divider from "../components/layout/Divider.svelte";
  import DialogHeader from "../components/headers/DialogHeader.svelte";

  let inputToken = "";

  async function createGame() {
    try {
      const res = await fetch("/api/games", {
        method: "PUT",
      });

      const newGame = (await res.json()) as Game;
      push(`/games/${newGame.token}`);
    } catch (err) {
      // TODO Handle all API errors in a generic way
    }
  }

  async function attendGame() {
    push(`/games/${inputToken}`);
  }
</script>

<Dialog>
  <DialogHeader>Start Game</DialogHeader>
  <SecondaryButton on:click={createGame}>Create new game</SecondaryButton>

  <Divider><span slot="text">or</span></Divider>

  <form on:submit|preventDefault={attendGame}>
    <div class="flex flex-col mb-6">
      <input
        id="token"
        name="token"
        placeholder="Token"
        bind:value={inputToken}
        class="text-sm sm:text-base placeholder-gray-500 rounded-lg border border-gray-400 w-full py-2 px-3 focus:outline-none focus:border-blue-400" />
    </div>
    <PrimaryButton on:click={attendGame}>Attend</PrimaryButton>
  </form>
  <div class="flex items-center mt-4">
    <div class="flex ml-auto">
      <a
        href="/"
        use:link
        class="inline-flex text-xs sm:text-sm text-blue-500 hover:text-blue-700">Back
        to Start</a>
    </div>
  </div>
</Dialog>
