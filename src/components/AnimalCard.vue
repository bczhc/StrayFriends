<script setup lang="ts">
import {computed, ref} from "vue";
import AdoptionRequest from "./AdoptionRequest.vue";
import {useRouter} from "vue-router";
import UserBox from "./UserBox.vue";
import AdoptionStatus from "./AdoptionStatus.vue";
import DateView from "./DateView.vue";
import {checkAdmin} from "../jwt.ts";
import {apiDelete, apiPatch} from "../api.ts";
import {useMessage} from 'naive-ui';
import {messageError} from "../main.ts";

const message = useMessage();

let emit = defineEmits(['imageClick', 'adoptionClick', 'userProfileClick', 'update'])
let router = useRouter();

let showAdoptionModal = ref(false);

function adoptionClick() {
  emit('adoptionClick');
  showAdoptionModal.value = true;
}

const props = defineProps<{
  coverImage?: string,
  name?: string,
  description?: string,
  userAvatarImage?: string,
  userId?: number,
  username?: string,
  loading?: boolean,
  postId?: number,
  adopted?: boolean,
  date?: string,
}>();

let loading = computed(() => props.loading);

type OptionKey = 'delete' | 'mark as adopted';

let operationsOptions: { key: OptionKey, label: string }[] = [
  {key: 'delete', label: '删除'},
  {key: 'mark as adopted', label: '标记为已领养'},
];

function operationsOnSelected(key: OptionKey) {
  let f = async () => {
    switch (key) {
      case "delete":
        await apiDelete(`/api/animal/${props.postId}`);
        break;
      case "mark as adopted":
        await apiPatch(`/api/animal/${props.postId}/adopt`);
        break;
    }
  };
  f().then(_r => {
    message.success('操作成功');
    emit('update');
  }).catch(e => messageError(e, message));
}
</script>

<template>
  <n-modal v-model:show="showAdoptionModal">
    <n-card
        style="width: 600px"
        title="申请领养"
        :bordered="false"
        size="huge"
        role="dialog"
        aria-modal="true"
    >
      <AdoptionRequest @cancel="showAdoptionModal = false"
                       @success="showAdoptionModal = false"
                       :post-id="props.postId"
      />
    </n-card>
  </n-modal>

  <div id="parent">
    <div class="center-parent">
      <img :src="props.coverImage" alt="image" style="max-width: 100%"
           @click="router.push(`/animal/${postId}`); emit('imageClick')"
           id="img"
           v-if="!loading"
      />
      <n-skeleton
          v-else
          height="150px"
      />
      <n-h2 class="label">
        <span v-if="!loading">{{ props.name }}</span>
        <n-skeleton width="50%" style="margin-top: .1em" v-else/>
      </n-h2>

    </div>
    <div style="padding: 5px">
      <n-text style="word-wrap: break-word">
        <span v-if="!loading">{{ props.description }}</span>
        <n-skeleton v-else/>
      </n-text>
    </div>
    <div id="bottom-flex">
      <UserBox :avatar-image="loading ? '' : props.userAvatarImage"
               :username="props.username"
               @click="emit('userProfileClick')"
               style="padding-top: .5em"
               :user-id="userId"
      />

      <div>
        <n-dropdown v-if="checkAdmin()" :options="operationsOptions" @select="operationsOnSelected" trigger="click">
          <n-button v-if="checkAdmin()" :disabled="loading">操作</n-button>
        </n-dropdown>
        <n-button @click="adoptionClick" :disabled="loading">申请领养</n-button>
      </div>
    </div>
    <div id="date-line">
      <DateView>{{ props.date }}</DateView>
      <AdoptionStatus v-if="props.adopted"/>
    </div>
  </div>
</template>

<style scoped lang="scss">
.label {
  margin: 0;
  padding: 0;
  display: flex;
  justify-content: center;
}

.center-parent {
  text-align: center;
}

#parent {
  display: inline-block;
  padding: .8em;
  width: 25em;
  background-color: white;
  transition: margin-top 0.2s ease-in-out;
  border-radius: 5px;
}

#parent:hover {
  margin-top: -4px;
  transition: 0.2s ease-in-out;
  box-shadow: rgba(255, 255, 255, 0.1) 0 1px 1px 0 inset, rgba(50, 50, 93, 0.25) 0 50px 100px -20px, rgba(0, 0, 0, 0.3) 0px 30px 60px -30px;
}

#bottom-flex {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

#img {
  cursor: pointer;
}

#date-line {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-top: .5em;
}
</style>
