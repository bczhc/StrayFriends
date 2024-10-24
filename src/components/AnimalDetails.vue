<script setup lang="ts">
import {useRoute} from "vue-router";
import Header from "./Header.vue";
import TextBanner from "./TextBanner.vue";
import {AnimalCardInfo, apiGet, imageUrl} from "../api.ts";
import {Ref, ref} from "vue";
import {useMessage} from 'naive-ui';
import UserBox from "./UserBox.vue";
import AdoptionRequest from "./AdoptionRequest.vue";
import DateView from "./DateView.vue";
import AdoptionStatus from "./AdoptionStatus.vue";
import {formatDate, messageError} from "../main.ts";
import {checkOwner} from "../jwt.ts";

const message = useMessage();

let route = useRoute();
let animalId = route.params['id'];

let animalInfo: Ref<AnimalCardInfo | null> = ref(null);

apiGet(`/api/animal/${animalId}`).then(r => {
  console.log(r);
  animalInfo.value = r as AnimalCardInfo;
}).catch(e => messageError(e, message));

let showAdoptionModal = ref(false);

type DropdownKey = 'delete';

let operationOptions: { key: DropdownKey, label: string }[] = [
  {key: 'delete', label: '删除'}
];

function operationOnSelected(key: DropdownKey) {
  switch (key) {
    case "delete":

      break;
  }
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
                       :post-id="animalId"
      />
    </n-card>
  </n-modal>

  <div id="root">
    <Header/>
    <TextBanner height="300px" text="动物详情" font-size="3em"/>
    <div id="content-div-wrapper" style="height: 100vh">
      <div id="content-div">
        <div class="h-center" v-if="!animalInfo">
          <n-h1>
            正在加载……
            <n-spin/>
          </n-h1>
        </div>
        <div v-else>
          <div id="content-line1">
            <UserBox :avatar-image="imageUrl(animalInfo.userAvatarImageId)"
                     :username="animalInfo.username"/>
            <div>
              <n-dropdown v-if="checkOwner(animalInfo.postUid)" :options="operationOptions"
                          @select="operationOnSelected">
                <n-button>操作</n-button>
              </n-dropdown>
              <n-button>爱心捐赠</n-button>
              <n-button @click="showAdoptionModal = true" :disabled="animalInfo.adopted">申请领养</n-button>
            </div>
          </div>
          <div id="content-line2">
            <DateView>{{ formatDate(new Date(animalInfo.creationTime * 1000)) }}</DateView>
            <AdoptionStatus v-if="animalInfo.adopted"/>
          </div>
          <n-h1>{{ animalInfo.name }}</n-h1>
          <n-text>{{ animalInfo.description }}</n-text>
          <img v-for="x in animalInfo.imageIdList"
               alt=""
               :src="imageUrl(x)"
               width="100%"
          />
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped lang="scss">
#banner {
  height: 100px;
  background-color: #c1633d;
  font-weight: lighter;
}

#content-div-wrapper {
  display: flex;
  justify-content: center;
  background-color: lightgray;
}

#content-div {
  padding: 2em;
  width: 60%;
  background-color: white;
}

#root {
}

#content-line1 {
  display: flex;
  justify-content: space-between;
}

#content-line2 {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-top: .5em;
}
</style>
