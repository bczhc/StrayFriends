<script setup lang="ts">
import {ref} from "vue";
import {CHECK_DIGITS, messageError} from "../main.ts";
import {useMessage} from "naive-ui";
import {apiPost} from "../api.ts";
import ImageUpload from "./ImageUpload.vue";

const message = useMessage();

let name = ref('');
let description = ref('');
let content = ref('');
let mobileNumber = ref('');

let emit = defineEmits(['cancel', 'success']);

let submitOnProgress = ref(false);
let uploadedImageIds = [];

function checkInput() {
  return name.value != ''
      && description.value != ''
      && uploadedImageIds.length !== 0
      && mobileNumber.value != '';
}

function submitClick() {
  if (!checkInput()) {
    message.info('请完整输入');
    return;
  }
  submitOnProgress.value = true;
  apiPost('/api/animal', {
    name: name.value,
    description: description.value,
    content: content.value,
    imageIdList: JSON.stringify(uploadedImageIds),
    mobileNumber: mobileNumber.value,
  }).then(_r => {
    message.success('发布成功');
    emit('success');
  }).catch(e => messageError(e, message)).finally(() => {
    submitOnProgress.value = false;
  });
}
</script>

<template>
  <n-form
      label-placement="left"
      label-width="auto"
      require-mark-placement="left"
  >
    <n-form-item label="名称" required>
      <n-input v-model:value="name" placeholder="为它起个名吧"/>
    </n-form-item>
    <n-form-item label="描述" required>
      <n-input v-model:value="description" placeholder="简短的描述"/>
    </n-form-item>
    <n-form-item label="内容">
      <n-input v-model:value="content" placeholder="详细展开说说"
               type="textarea"
      />
    </n-form-item>
    <n-form-item label="上传图片" required>
      <ImageUpload
          @update="list => uploadedImageIds = list"/>
    </n-form-item>
    <n-form-item label="联系方式" required>
      <n-input :allow-input="CHECK_DIGITS" v-model:value="mobileNumber"
               placeholder="手机号码"/>
    </n-form-item>
  </n-form>
  <n-space justify="end">
    <n-button type="primary" secondary @click="emit('cancel')">取消</n-button>
    <n-button type="primary" @click="submitClick">提交</n-button>
  </n-space>
</template>

<style scoped lang="scss">

</style>
