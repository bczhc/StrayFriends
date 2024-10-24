<script setup lang="ts">
import {Ref, ref} from "vue";
import {CHECK_DIGITS} from "../main.ts";
import {UploadFileInfo, useMessage} from "naive-ui";
import {apiPost, authHeaders, imageUrl, parseNUploadOnFinishEvent} from "../api.ts";

const message = useMessage();

let name = ref('');
let description = ref('');
let content = ref('');
let mobileNumber = ref('');

let emit = defineEmits(['cancel', 'success']);

let imagePreviewUrl = ref('');
let showImageUploadingModal = ref(false);
let imageFileList: Ref<UploadFileInfo[]> = ref([]);
let submitOnProgress = ref(false);

let naiveId2ServerId: Map<string, string> = new Map();

function uploadedImageIds() {
  return imageFileList.value.map(x => naiveId2ServerId.get(x.id)!!);
}

function checkInput() {
  return name.value != ''
      && description.value != ''
      && imageFileList.value.length !== 0
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
    imageIdList: JSON.stringify(uploadedImageIds()),
    mobileNumber: mobileNumber.value,
  }).then(r => {
    if (r.success()) {
      message.success('发布成功');
      emit('success');
    } else {
      message.error(r.messageOrEmpty());
    }
  }).catch(e => {
    message.error(e.toString());
  }).finally(() => {
    submitOnProgress.value = false;
  });
}
</script>

<template>
  <n-modal
      v-model:show="showImageUploadingModal"
      preset="card"
      style="width: 80%"
      title="预览"
  >
    <img :src="imagePreviewUrl" style="width: 100%" alt=""/>
  </n-modal>

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
      <n-upload
          action="/api/image/upload"
          :headers="authHeaders()"
          v-model:file-list="imageFileList"
          list-type="image-card"
          @preview="(file: UploadFileInfo) => {
            let serverImageId = naiveId2ServerId.get(file.id)!!;
            imagePreviewUrl = imageUrl(serverImageId);
            showImageUploadingModal = true;
          }"
          @finish="x => {
            let digest = parseNUploadOnFinishEvent(x.event).digest;
            console.log([x, digest]);
            naiveId2ServerId.set(x.file.id, digest);
          }"
      >
      </n-upload>
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
