<script setup lang="ts">

import Header from "./Header.vue";
import AnimalCard from "./AnimalCard.vue";
import {computed, ref} from "vue";
import PostAnimal from "./PostAnimal.vue";
import {AnimalCardInfo, apiGet, imageUrl} from "../api.ts";
import {useMessage} from 'naive-ui';
import TextBanner from "./TextBanner.vue";
import {formatDate} from "../main.ts";

const message = useMessage();

let showPostAnimalModal = ref(false);

let animalsLoading = ref(true);

let animalCardInfoList = ref<AnimalCardInfo[]>([])

let pageSize = 10;
let page = ref(1);
let animalCount = ref(pageSize);

let pageCount = computed(() => {
  if (animalCount.value % pageSize === 0) {
    return animalCount.value / pageSize;
  }
  return Math.floor(animalCount.value / pageSize) + 1;
});

function fetchAndUpdateAnimals() {
  animalsLoading.value = true;
  apiGet(`/api/animals?offset=${(page.value - 1) * pageSize}&limit=${pageSize}`)
      .then(r => {
        if (r.success()) {
          let list = r.data['animals'] as AnimalCardInfo[];
          animalCount.value = r.data['total'];
          console.log(list);
          animalCardInfoList.value = list;
          animalsLoading.value = false;
        } else {
          message.error(r.messageOrEmpty())
        }
      }).catch(e => {
    message.error(e.toString())
  });
}

fetchAndUpdateAnimals();
</script>

<template>
  <n-modal v-model:show="showPostAnimalModal">
    <n-card
        style="width: 600px"
        title="发布"
        :bordered="false"
        size="huge"
        role="dialog"
        aria-modal="true"
    >
      <PostAnimal
          @cancel="showPostAnimalModal = false"
          @success="showPostAnimalModal = false; fetchAndUpdateAnimals()"
      />
    </n-card>
  </n-modal>

  <Header/>
  <TextBanner height="300px" text="动物信息" font-size="5em"/>
  <div id="card-layout-wrapper">
    <div id="card-layout-wrapper-header">
      <div class="left-div">
        <span>最新发布</span>
      </div>
      <div class="right-div">
        <n-button type="primary" @click="showPostAnimalModal = true">发布</n-button>
      </div>
    </div>
    <div id="card-layout">
      <!-- show skeletons -->
      <AnimalCard v-if="animalsLoading"
                  v-for="() in Array(pageSize)"
                  loading
                  :post-id="0"
      />
      <AnimalCard
          v-else
          v-for="x in animalCardInfoList"
          :cover-image="imageUrl(x.imageIdList[0])"
          :description="x.description"
          :name="x.name"
          :user-avatar-image="imageUrl(x.userAvatarImageId)"
          :username="x.username"
          :loading="false"
          :post-id="x.postId"
          :adopted="x.adopted"
          :date="formatDate(new Date(x.creationTime * 1000))"
      />
    </div>
    <n-pagination v-model:page="page" :page-count="pageCount"
                  @update:page="fetchAndUpdateAnimals"
    />
  </div>
</template>

<style scoped lang="scss">
#card-layout {
  display: flex;
  flex-wrap: wrap;
  gap: 1em;
  max-height: 100%;
  box-sizing: border-box;
  justify-content: flex-start;
  align-items: flex-start;
  align-content: flex-start;
}

#card-layout-wrapper {
  padding: 4em;
  background-color: lightgray;
}

#card-layout-wrapper-header {
  padding-bottom: .5em;
  display: flex;
  justify-content: space-between;

  .right-div {
    margin-right: 2em;
  }
}

#card-layout-wrapper-header span {
  padding: .5em;
  display: inline-flex;
  align-items: center;
  border-bottom: 2px solid #f35a21;
}

</style>
