<script setup lang="ts">

import {authHeaders, imageUrl, parseNUploadOnFinishEvent} from "../api.ts";
import {UploadFileInfo} from "naive-ui";
import {Ref, ref} from "vue";

let imagePreviewUrl = ref('');
let showImageUploadingModal = ref(false);
let naiveId2ServerId: Map<string, string> = new Map();
let imageFileList: Ref<UploadFileInfo[]> = ref([]);

let emit = defineEmits<{
  update: [ids: string[]],
}>();

function uploadedImageIds() {
  return imageFileList.value.map(x => naiveId2ServerId.get(x.id)!!);
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
            emit('update', uploadedImageIds());
          }"
  >
  </n-upload>
</template>

<style scoped lang="scss">

</style>
