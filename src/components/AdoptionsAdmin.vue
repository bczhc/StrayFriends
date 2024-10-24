<script setup lang="ts">

import Header from "./Header.vue";
import TextBanner from "./TextBanner.vue";
import AdoptionRequestListView from "./AdoptionRequestListView.vue";
import {AdoptionRequest, apiGet} from "../api.ts";
import {useMessage} from 'naive-ui';
import {ref} from "vue";
import {messageError} from "../main.ts";

const message = useMessage();

const pageSize = 20;

let totalCount = ref(0);
let list = ref<AdoptionRequest[]>([]);
let page = ref(0);

async function fetch() {
  let r = await apiGet('/api/adoptions/count');
  totalCount.value = r;

  r = await apiGet(`/api/adoptions/list?offset=${page.value * pageSize}&limit=${pageSize}`);
  console.log(r);
  let list = r as AdoptionRequest[];
  for (let x of list) {
    let animal = await apiGet(`/api/animal/${x.animalPostId}`);
    console.log(animal);
  }
}

fetch().catch(e => messageError(e, message));
</script>

<template>
  <Header/>
  <TextBanner height="100px" text="申请批准" font-size="3em"/>

  <div id="adoption-list">
    <AdoptionRequestListView/>
    <AdoptionRequestListView/>
    <AdoptionRequestListView/>
  </div>
</template>

<style scoped lang="scss">
#adoption-list {
  display: flex;
  flex-direction: column;
}
</style>
