<script setup lang="ts">
import {apiGet, apiPut, authHeaders} from "../api.ts";
import {Ref, ref} from "vue";
import PasswordConfirm from "./PasswordConfirm.vue";
import {useMessage} from 'naive-ui';

let message = useMessage();

type GenderValue = 'male' | 'female' | 'secret' | 'other';

let name = ref('');
let oldPassword = ref('');
let newPassword = ref('');
let newPasswordConfirm = ref('');
let genderOptions: { label: string, value: GenderValue }[] = [
  {
    label: '男',
    value: 'male',
  },
  {
    label: '女',
    value: 'female',
  },
  {
    label: '秘密',
    value: 'secret',
  },
  {
    label: '其他',
    value: 'other',
  }
];
let selectedGender: Ref<GenderValue> = ref('secret');

let avatarImageId = ref('');

let loaded = ref(false);

apiGet('/api/me').then(r => {
  if (r.success()) {
    name.value = r.data['name'];
    loaded.value = true;
  } else {
    message.error(r.messageOrEmpty());
  }
}).catch(e => {
  message.error(e.toString());
});

function updateInfoClick() {
  apiPut('/')
}

</script>

<template>
  <n-form id="form" size="large" :disabled="!loaded"
          label-placement="left"
          label-width="auto"
  >
    <n-form-item label="昵称">
      <n-input v-model:value="name"/>
    </n-form-item>
    <n-form-item label="性别">
      <n-grid cols="2">
        <n-gi>
          <n-select :options="genderOptions" v-model:value="selectedGender"/>
        </n-gi>
        <n-gi v-if="selectedGender === 'other'">
          <n-input/>
        </n-gi>
      </n-grid>
    </n-form-item>
    <n-form-item label="头像">
      <n-upload
          action="/api/image/upload"
          :headers="authHeaders()"
          @finish="x => {
            let resText = (x.event.target as XMLHttpRequest).response;
            let res = JSON.parse(resText);
            let digest = res['data'];
            avatarImageId = digest;
            console.log(digest);
            console.log(res);
          }"
      >
        <n-avatar size="large" round
                  :src="`/api/image/${avatarImageId}`"/>
      </n-upload>
    </n-form-item>
    <n-form-item label="原密码">
      <n-input type="password" v-model:value="oldPassword"/>
    </n-form-item>
    <PasswordConfirm
        v-model:password="newPassword"
        v-model:password-confirm="newPasswordConfirm"
    />
    <n-space justify="space-evenly" size="large">
      <n-button size="large" type="primary" secondary @click="updateInfoClick">更新</n-button>
    </n-space>
  </n-form>
</template>

<style scoped lang="scss">

</style>
