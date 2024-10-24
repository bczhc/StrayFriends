<script setup lang="ts">
import {AnimalCardInfo, apiGet, imageUrl, User} from "../api.ts";
import {messageError} from "../main.ts";
import {useMessage} from 'naive-ui';
import {ref, Ref} from "vue";
import UserBox from "./UserBox.vue";

const message = useMessage();

const props = defineProps<{
  postUid: number,
  animalId: number,
  mobileNumber: string,
  requestDetails: string,
}>();

let animal: Ref<AnimalCardInfo | null> = ref(null);
let user: Ref<User | null> = ref(null);

async function fetch() {
  user.value = await apiGet(`/api/user/${props.postUid}`);
  console.log(user.value);

  animal.value = await apiGet(`/api/animal/${props.animalId}`);
  console.log(animal.value);
}

fetch().catch(e => messageError(e, message));
</script>

<template>
  <div id="root" v-if="animal">
    <img
        alt=""
        :src="imageUrl(animal!!.imageIdList[0])"
        height="100px"
    />
    <div>
      <div id="right-div">
        <UserBox
            v-if="user"
            :avatar-image="imageUrl(user.avatarImageId)"
            :username="user.name"
            :mobile-number="mobileNumber"
        />
        <n-button>操作</n-button>
      </div>
      <n-divider style="margin: 5px; padding: 0"/>
      <span id="details">{{ requestDetails }}</span>
    </div>
  </div>
</template>

<style scoped lang="scss">
#root {
  display: flex;
  padding: 10px;

  #right-div {
    display: flex;
    justify-content: space-between;
  }
}

#root > div:nth-child(2) {
  width: 100%;
  display: flex;
  flex-direction: column;
  padding: 5px 10px;
}
</style>
