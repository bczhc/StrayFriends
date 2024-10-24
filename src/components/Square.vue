<script setup lang="ts">
import {useMessage} from 'naive-ui';
import Header from "./Header.vue";
import TextBanner from "./TextBanner.vue";
import SquarePost from "./SquarePost.vue";
import {apiGet, apiPost} from "../api.ts";
import {messageError, paginationCount} from "../main.ts";
import {ref} from "vue";
import ImageUpload from "./ImageUpload.vue";

const message = useMessage();

let squareList = ref<SquarePost[]>([]);
let page = ref(1);
const pageSize = 20;
let total = ref(0);

async function fetch() {
  let r = await apiGet(`/api/square?offset=${(page.value - 1) * pageSize}&limit=${pageSize}`);
  squareList.value = r['list'] as SquarePost[];
  total.value = r['total'];
  console.log(r);
}

fetch().catch(e => messageError(e, message));

let postContent = ref('');
let postImages = ref<string[]>([]);

let posting = ref(false);

function postNew() {
  if (!postContent.value) {
    message.error('请输入内容');
    return;
  }

  posting.value = true;
  apiPost('/api/square', {
    content: postContent.value,
    images: JSON.stringify(postImages.value),
  })
      .then(() => {
        message.success('发送成功');
        postContent.value = '';
        postImages.value = [];
        fetch();
      })
      .catch(e => messageError(e, message))
      .finally(() => posting.value = false);
}
</script>

<template>
  <Header/>
  <TextBanner height="100px" text="广场" font-size="3em"/>

  <div id="root">
    <div id="post-list">
      <div v-for="x in squareList">
        <SquarePost :square="x"/>
        <n-divider style="margin: .5em; padding: 0"/>
      </div>
    </div>

    <div id="bottom-line">
      <n-pagination v-model:page="page" :page-count="paginationCount(total, pageSize)"
                    @update:page="fetch"/>
    </div>
    <div id="post-box">
      <n-input type="textarea" placeholder="我也来发布" v-model:value="postContent"/>
      <ImageUpload @update="x => postImages = x"/>
      <n-button @click="postNew" :loading="posting">提交</n-button>
    </div>
  </div>
</template>

<style scoped lang="scss">
#post-list {
  display: flex;
  flex-direction: column;
  gap: .5em;
  padding: 1em;
}

#bottom-line {
  padding: 1em;
}

#post-box {
  display: flex;
  flex-direction: column;
  align-items: center;
}

#root {
}
</style>
