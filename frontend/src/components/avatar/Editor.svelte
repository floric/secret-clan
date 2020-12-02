<script lang="typescript">
  import ArrowLeft from "../buttons/ArrowLeft.svelte";
  import ArrowRight from "../buttons/ArrowRight.svelte";

  type BodyParts = "Legs" | "Body" | "Hands" | "Arms" | "Head" | "Face";
  const allCharacters = [
    "Artist",
    "Astrologer",
    "Blacksmith",
    "Citizen",
    "Herbalist",
    "Hunter",
    "Jeweler",
    "Sage",
    "Warlord",
  ];
  const allParts: Record<BodyParts, { index: number; label: string }> = {
    Legs: { index: 0, label: "Legs" },
    Body: { index: 0, label: "Body" },
    Hands: { index: 0, label: "Hands" },
    Arms: { index: 0, label: "Arms" },
    Head: { index: 0, label: "Head" },
    Face: { index: 0, label: "Face" },
  };
  const selectPrevious = (key: BodyParts) => () =>
    (allParts[key].index =
      allParts[key].index === 0
        ? allCharacters.length - 1
        : allParts[key].index - 1);
  const selectNext = (key: BodyParts) => () =>
    (allParts[key].index =
      allParts[key].index < allCharacters.length - 1
        ? allParts[key].index + 1
        : 0);
</script>

<div class="flex justify-center items-center">
  <div
    style="width: 24rem; height: 24rem;"
    class="flex justify-center items-center">
    <div
      class="relative w-full h-full min-h-full min-w-full flex justify-center">
      <img
        alt="Right leg"
        src="/static/images/{allCharacters[allParts.Legs.index]}/PNG/Vector Parts/Right_Leg.png"
        class="absolute"
        style="height: 20%; left: 35%; top: 69%;" />
      <img
        alt="Left leg"
        src="/static/images/{allCharacters[allParts.Legs.index]}/PNG/Vector Parts/Left_Leg.png"
        class="absolute"
        style="height: 20%; right: 35%; top: 69%;" />
      <img
        alt="Body"
        src="/static/images/{allCharacters[allParts.Body.index]}/PNG/Vector Parts/Body.png"
        class="absolute"
        style="height: 50%; top: 40%;" />
      <img
        alt="Left Hand"
        src="/static/images/{allCharacters[allParts.Hands.index]}/PNG/Vector Parts/Left_Hand.png"
        class="absolute"
        style="height: 20%; left: 54%; top: 58%;" />
      <img
        alt="Right Hand"
        src="/static/images/{allCharacters[allParts.Hands.index]}/PNG/Vector Parts/Right_Hand.png"
        class="absolute"
        style="height: 20%; right: 54%; top: 58%;" />
      <img
        alt="Left Arm"
        src="/static/images/{allCharacters[allParts.Arms.index]}/PNG/Vector Parts/Left_Arm.png"
        class="absolute"
        style="height: 20%; left: 52%; top: 53%;" />
      <img
        alt="Right Arm"
        src="/static/images/{allCharacters[allParts.Arms.index]}/PNG/Vector Parts/Right_Arm.png"
        class="absolute"
        style="height: 20%; right: 52%; top: 53%;" />
      <img
        alt="Head"
        src="/static/images/{allCharacters[allParts.Head.index]}/PNG/Vector Parts/Head.png"
        class="absolute"
        style="height: 50%; left: 25%; top: 15%;" />
      <img
        alt="Face"
        src="/static/images/{allCharacters[allParts.Face.index]}/PNG/Vector Parts/Face_01.png"
        class="absolute"
        style="height: 20%; left: 37%; top: 38%;" />
    </div>
  </div>
  <div class="flex-1">
    <div class="flex justify-between mb-2">
      {allParts.Head.label}:

      <div class="flex justify-between items-center w-48">
        <ArrowLeft on:click={selectPrevious('Head')} />
        {allCharacters[allParts.Head.index]}
        <ArrowRight on:click={selectNext('Head')} />
      </div>
    </div>
    <div class="flex justify-between mb-2">
      {allParts.Face.label}:

      <div class="flex justify-between items-center w-48">
        <ArrowLeft on:click={selectPrevious('Face')} />
        {allCharacters[allParts.Face.index]}
        <ArrowRight on:click={selectNext('Face')} />
      </div>
    </div>
    <div class="flex justify-between mb-2">
      {allParts.Body.label}:

      <div class="flex justify-between items-center w-48">
        <ArrowLeft on:click={selectPrevious('Body')} />
        {allCharacters[allParts.Body.index]}
        <ArrowRight on:click={selectNext('Body')} />
      </div>
    </div>
    <div class="flex justify-between mb-2">
      {allParts.Arms.label}:

      <div class="flex justify-between items-center w-48">
        <ArrowLeft on:click={selectPrevious('Arms')} />
        {allCharacters[allParts.Arms.index]}
        <ArrowRight on:click={selectNext('Arms')} />
      </div>
    </div>
    <div class="flex justify-between mb-2">
      {allParts.Legs.label}:

      <div class="flex justify-between items-center w-48">
        <ArrowLeft on:click={selectPrevious('Legs')} />
        {allCharacters[allParts.Legs.index]}
        <ArrowRight on:click={selectNext('Legs')} />
      </div>
    </div>
    <div class="flex justify-between mb-2">
      {allParts.Hands.label}:

      <div class="flex justify-between items-center w-48">
        <ArrowLeft on:click={selectPrevious('Hands')} />
        {allCharacters[allParts.Hands.index]}
        <ArrowRight on:click={selectNext('Hands')} />
      </div>
    </div>
  </div>
</div>
