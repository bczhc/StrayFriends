<script setup lang="ts">

import Header from "./Header.vue";
import TextBanner from "./TextBanner.vue";
import AdoptionRequestListView from "./AdoptionRequestListView.vue";
import {AdoptionRequest, apiGet} from "../api.ts";
import {useMessage} from 'naive-ui';
import {ref} from "vue";

const message = useMessage();

const pageSize = 20;

let totalCount = ref(0);
let list = ref<AdoptionRequest[]>([]);
let page = ref(0);

async function fetch() {
  let r = await apiGet('/api/adoptions/count');
  if (r.success()) {
    totalCount.value = r.data;
  } else {
    message.error(r.messageOrEmpty());
  }

  r = await apiGet(`/api/adoptions/list?offset=${page.value * pageSize}&limit=${pageSize}`);
  if (r.success()) {
    console.log(r.data);
    let list = r.data as AdoptionRequest[];
    for (let x of list) {
      let animal = await apiGet(`/api/animal/${x.animalPostId}`);
      // TODO
    }
  } else {
    message.error(r.messageOrEmpty());
  }
}

fetch().catch(e => message.error(e.toString()));
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
