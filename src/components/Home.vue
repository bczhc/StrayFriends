<script setup lang="ts">

import Header from "./Header.vue";
import AnimalCard from "./AnimalCard.vue";
</script>

<template>
  <Header/>
  <div id="card-layout">
    <AnimalCard/>
    <AnimalCard/>
    <AnimalCard/>
    <AnimalCard/>
    <AnimalCard/>
    <AnimalCard/>
    <AnimalCard/>
  </div>
</template>

<style scoped lang="scss">
#card-layout {
  display: flex;
  flex-wrap: wrap;
  gap: 1em;
  padding: 4em;
  max-height: 100%;
  background-color: lightgray;
  box-sizing: border-box;
  justify-content: flex-start;
  align-items: flex-start;
  align-content: flex-start;
}

</style>
