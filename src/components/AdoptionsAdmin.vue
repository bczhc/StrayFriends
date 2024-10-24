<script setup lang="ts">

import Header from "./Header.vue";
import TextBanner from "./TextBanner.vue";
import AdoptionRequestListView from "./AdoptionRequestListView.vue";
import {AdoptionRequest, apiGet} from "../api.ts";
import {useMessage} from 'naive-ui';
import {ref} from "vue";
import {messageError, paginationCount} from "../main.ts";

const message = useMessage();

const pageSize = 20;

let totalCount = ref(0);
let list = ref<AdoptionRequest[]>([]);
let page = ref(1);

async function fetch() {
  let r = await apiGet('/api/adoptions/count');
  totalCount.value = r;

  r = await apiGet(`/api/adoptions/list?offset=${(page.value - 1) * pageSize}&limit=${pageSize}`);
  list.value = r;
  console.log(r);
}

fetch().catch(e => messageError(e, message));
</script>

<template>
  <Header/>
  <TextBanner height="100px" text="申请批准" font-size="3em"/>

  <div id="adoption-list">
    <AdoptionRequestListView
        v-for="x in list"
        :request-id="x.requestId"
        :animal-id="x.animalPostId"
        :post-uid="x.postUid"
        :mobile-number="x.mobileNumber"
        :request-details="x.requestDetails"
        class="list-view"
        @update="fetch"
    />
  </div>
  <n-pagination v-model:page="page"
                :page-count="paginationCount(totalCount, pageSize)"
                id="pagination"
  />
</template>

<style scoped lang="scss">
#adoption-list {
  display: flex;
  flex-direction: column;
  gap: 10px;
  padding: 10px;
}

#pagination {
  padding: 0 10px;
}
</style>
