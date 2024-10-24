<script setup lang="ts">
import {AnimalCardInfo, apiDelete, apiGet, apiPatch, imageUrl, User} from "../api.ts";
import {messageError} from "../main.ts";
import {DialogOptions, useDialog, useMessage} from 'naive-ui';
import {ref, Ref} from "vue";
import UserBox from "./UserBox.vue";
import {useRouter} from "vue-router";

const router = useRouter();
const message = useMessage();
const dialog = useDialog();

const props = defineProps<{
  requestId: number,
  postUid: number,
  animalId: number,
  mobileNumber: string,
  requestDetails: string,
}>();

let emit = defineEmits(['update']);

let animal: Ref<AnimalCardInfo | null> = ref(null);
let user: Ref<User | null> = ref(null);

async function fetch() {
  user.value = await apiGet(`/api/user/${props.postUid}`);
  console.log(user.value);

  animal.value = await apiGet(`/api/animal/${props.animalId}`);
  console.log(animal.value);
}

fetch().catch(e => messageError(e, message));

type Key = 'delete' | 'approve';

let operationOptions: { key: Key, label: string }[] = [
  {key: 'delete', label: '删除'},
  {key: 'approve', label: '同意'},
];

async function deleteRequest() {
  return await apiDelete(`/api/adoption/${props.requestId}`);
}

async function markAnimalAsAdopted() {
  return await apiPatch(`/api/animal/${props.animalId}/adopt`);
}

function deleteConfirm() {
  let d: DialogOptions;
  d = dialog.warning({
    title: '删除',
    content: '是否删除？',
    positiveText: '是',
    negativeText: '否',
    onPositiveClick: () => {
      d.loading = true;
      return new Promise(resolve => {
        deleteRequest().catch(e => messageError(e, message)).then(_r => {
          message.success('删除成功');
          emit('update');
          d.loading = false;
          resolve();
        });
      });
    }
  });
}

function approveConfirm() {
  let d: DialogOptions;
  d = dialog.warning({
    title: '同意',
    content: '是否标记为已领养？',
    positiveText: '是',
    negativeText: '否',
    onPositiveClick: () => {
      d.loading = true;
      return new Promise(resolve => {
        // approve and then delete the request
        const f = async () => {
          await markAnimalAsAdopted();
          await deleteRequest();
        };
        f().catch(e => messageError(e, message)).then(_r => {
          message.success('操作成功');
          d.loading = false;
          emit('update');
          resolve();
        })
      });
    }
  });
}

function operationClick(key: Key) {
  switch (key) {
    case "delete":
      deleteConfirm();
      break;
    case "approve":
      approveConfirm();
      break;
  }
}
</script>

<template>
  <div id="root" v-if="animal">
    <img
        alt=""
        :src="imageUrl(animal!!.imageIdList[0])"
        height="100px"
        @click="router.push(`/animal/${animalId}`)"
    />
    <div>
      <div id="right-div">
        <UserBox
            v-if="user"
            :avatar-image="imageUrl(user.avatarImageId)"
            :username="user.name"
            :mobile-number="mobileNumber"
        />
        <n-dropdown trigger="click" :options="operationOptions" @select="operationClick">
          <n-button>操作</n-button>
        </n-dropdown>
      </div>
      <n-divider style="margin: 5px; padding: 0"/>
      <span id="details">{{ requestDetails }}</span>
    </div>
  </div>
</template>

<style scoped lang="scss">
#root {
  display: flex;

  #right-div {
    display: flex;
    justify-content: space-between;
  }

  img {
    cursor: pointer;
  }
}

#root > div:nth-child(2) {
  width: 100%;
  display: flex;
  flex-direction: column;
  padding: 5px 10px;
}
</style>
