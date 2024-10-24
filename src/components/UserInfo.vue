<script setup lang="ts">
import {apiGet, apiPut, authHeaders, imageUrl, parseNUploadOnFinishEvent} from "../api.ts";
import {Ref, ref} from "vue";
import PasswordConfirm from "./PasswordConfirm.vue";
import {useMessage} from 'naive-ui';
import {messageError} from "../main.ts";

let message = useMessage();

type GenderValue = 'male' | 'female' | 'secret' | 'other';

let updating = ref(false);
let name = ref('');
let oldPassword = ref('');
let newPassword = ref('');
let newPasswordConfirm = ref('');
let bio: Ref<string | null> = ref(null);
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
let genderOther = ref('');

let avatarImageId: Ref<string | null> = ref(null);

let loaded = ref(false);

apiGet('/api/me').then(r => {
  let data = r;
  console.log(data);
  name.value = data['name'];
  selectedGender.value = data['genderType'];
  genderOther.value = data['genderOther'];
  avatarImageId.value = data['avatarImageId'];
  bio.value = data['bio'];
  loaded.value = true;
}).catch(e => messageError(e, message));

function updateInfoClick() {
  updating.value = true;
  apiPut('/api/me', {
    name: name.value,
    oldPassword: oldPassword.value,
    newPassword: newPassword.value,
    avatarImageId: avatarImageId.value,
    genderType: selectedGender.value,
    genderOther: genderOther.value,
    bio: bio.value,
  }).then(_r => {
    message.success('更新成功');
  }).catch(x => messageError(x, message)).finally(() => updating.value = false);
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
          <n-input v-model:value="genderOther"/>
        </n-gi>
      </n-grid>
    </n-form-item>
    <n-form-item label="头像">
      <n-upload
          action="/api/image/upload"
          :headers="authHeaders()"
          @finish="x => {
            let digest = parseNUploadOnFinishEvent(x.event).digest;
            avatarImageId = digest;
            console.log(digest);
            console.log(res);
          }"
      >
        <n-avatar size="large" round
                  :src="imageUrl(avatarImageId)"/>
      </n-upload>
    </n-form-item>
    <n-form-item label="原密码">
      <n-input type="password" v-model:value="oldPassword"/>
    </n-form-item>
    <PasswordConfirm
        v-model:password="newPassword"
        v-model:password-confirm="newPasswordConfirm"
    />
    <n-form-item label="个人简介">
      <n-input type="textarea" v-model:value="bio"/>
    </n-form-item>
    <n-space justify="space-evenly" size="large">
      <n-button size="large" type="primary" secondary
                :disabled="updating"
                @click="updateInfoClick">更新
      </n-button>
    </n-space>
  </n-form>
</template>

<style scoped lang="scss">

</style>
