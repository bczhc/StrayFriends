<script setup lang="ts">

import Demo1 from "./components/Demo1.vue";
import axios from "axios";

async function click() {
  let a = await axios.get('/api/routes');
  console.log(a);
}
</script>

<template>
  <h1>hello,</h1>
  <Demo1/>

  <n-button @click="click">Button</n-button>
</template>

<style scoped>

</style>
