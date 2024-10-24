<script setup lang="ts">
import {useMessage} from 'naive-ui';
import {apiGet, imageUrl, SquarePost, User} from "../api.ts";
import {computed, ref, Ref} from "vue";
import {formatDate, messageError} from "../main.ts";
import UserBox from "./UserBox.vue";

const message = useMessage();

const props = defineProps<{
  square: SquarePost,
}>();

let square = computed(() => props.square);
let userRef: Ref<User | null> = ref(null);

async function fetch() {
  userRef.value = await apiGet(`/api/user/${square.value.postUid}`);
}

fetch().then().catch(e => messageError(e, message));
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
</style>
