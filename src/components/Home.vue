<script setup lang="ts">

import Header from "./Header.vue";
import AnimalCard from "./AnimalCard.vue";
import {ref} from "vue";
import PostAnimal from "./PostAnimal.vue";
import {apiGet, imageUrl} from "../api.ts";
import {useMessage} from 'naive-ui';

const message = useMessage();

let showPostAnimalModal = ref(false);

interface AnimalCardInfo {
  username: string,
  userAvatarImageId: string,
  name: string,
  description: string,
  content: string,
  creationTime: number,
  imageIdList: string[],
}

let animalsLoading = ref(true);

let animalCardInfoList = ref<AnimalCardInfo[]>([])

// let queryOffset = ref(0);
// let queryLimit = ref(0);

let queryOffset = 0;
let queryLimit = 10;

apiGet(`/api/animals?offset=${queryOffset}&limit=${queryLimit}`)
    .then(r => {
      if (r.success()) {
        let list = r.data as AnimalCardInfo[];
        console.log(list);
        animalCardInfoList.value = list;
        animalsLoading.value = false;
      } else {
        message.error(r.messageOrEmpty())
      }
    }).catch(e => {
  message.error(e.toString())
})
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
          @success="showPostAnimalModal = false"
      />
    </n-card>
  </n-modal>

  <Header/>
  <div id="header">
    <n-text class="display-large">
      动物信息
    </n-text>
  </div>
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
                  v-for="() in Array(queryLimit)"
                  loading
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
      />
    </div>
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

#header {
  height: 300px;
  display: flex;
  background-color: #c1633d;
  justify-content: center;
  align-items: center;
}

.display-large {
  font-size: 5em;
  font-weight: lighter;
  color: white;
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
