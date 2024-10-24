<script setup lang="ts">
import {useDialog, useMessage} from 'naive-ui';
import {apiDelete, apiGet, imageUrl, SquarePost, User} from "../api.ts";
import {computed, ref, Ref} from "vue";
import {confirmApiRequest, formatDate, messageError} from "../main.ts";
import UserBox from "./UserBox.vue";
import {checkAdmin, checkOwned} from "../jwt.ts";
import {useRouter} from "vue-router";

const message = useMessage();
const dialog = useDialog();
const router = useRouter();

const props = defineProps<{
  square: SquarePost,
}>();

let square = computed(() => props.square);
let userRef: Ref<User | null> = ref(null);
let owned = computed(() => checkOwned(square.value.postUid));

async function fetch() {
  userRef.value = await apiGet(`/api/user/${square.value.postUid}`);
  if (!checkAdmin() && !owned.value) {
    // no permission to operate
    dropdownOptions = [];
  }
}

fetch().then().catch(e => messageError(e, message));

type DropdownKey = 'delete';

let dropdownOptions: { key: DropdownKey, label: string }[] = [
  {key: 'delete', label: '删除'},
];

function dropdownOnSelected(key: DropdownKey) {
  switch (key) {
    case "delete":
      confirmApiRequest(
          dialog,
          '删除',
          '是否删除？',
          finish => {
            apiDelete(`/api/square/${square.value.id}`)
                .then(() => {
                  message.success('操作成功');
                  // TODO: avoid hard refreshing the page
                  //  a way to go: notify the parent -> re-fetch
                  router.go();
                })
                .catch(e => messageError(e, message))
                .finally(() => finish())
          }
      );
      break;
  }
}
</script>

<template>
  <div id="root">
    <div id="left-div">
      <div>
        <UserBox v-if="userRef"
                 :username="userRef.name"
                 :avatar-image="imageUrl(userRef.avatarImageId)"
        />
        <n-text id="content">
          <span>{{ square.content }}</span><br>
          <n-image-group>
            <n-image v-for="x in square.images"
                     :src="imageUrl(x)"
                     height="50px"
                     style="margin-right: 5px"
            />
          </n-image-group>
        </n-text>
      </div>
    </div>
    <div id="right-div">
      <span>{{ formatDate(new Date(square.creationTime * 1000)) }}</span>
      <n-dropdown :options="dropdownOptions" @select="dropdownOnSelected">
        <span style="margin-left: 4px" class="three-dots"></span>
      </n-dropdown>
    </div>
  </div>
</template>

<style scoped lang="scss">
#root {
  display: flex;
  gap: 10px;
  justify-content: space-between;
}

#right-div {
  display: flex;
  align-items: flex-end;
}

#left-div > div {
  display: inline-flex;
  align-items: center;
  gap: 1em;
}

#content {
  word-wrap: break-word;
}

.three-dots:after {
  content: '\2807';
}
</style>
