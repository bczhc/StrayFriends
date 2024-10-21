<script setup lang="ts">
import {Component, h} from "vue";
import {NIcon} from "naive-ui";
import {LogOutOutline, PersonCircleOutline} from "@vicons/ionicons5";
import {useRoute, useRouter} from "vue-router";
import {JWT_CLEAR} from "../jwt.ts";

let router = useRouter();

function renderIcon(icon: Component) {
  return () => {
    return h(NIcon, null, {
      default: () => h(icon)
    });
  };
}

type DropdownKeys = 'my info' | 'logout';

let dropdownOptions = [
  {
    label: '我的信息',
    icon: renderIcon(PersonCircleOutline),
    key: 'my info'
  },
  {
    label: '退出登录',
    icon: renderIcon(LogOutOutline),
    key: 'logout'
  }
];

function onDropdownSelected(key: DropdownKeys) {
  switch (key) {
    case "my info":
      router.push('/me');
      break;
    case "logout":
      JWT_CLEAR();
      router.push({path: '/', query: {type: 'logout'}})
      break;
  }
}
</script>

<template>
  <n-page-header
      class="header"
  >
    <template #title>
      <div style="position: absolute">
        <span class="ver-center" id="header-title">主页</span>
        <img src="/icon.png" alt="" class="icon-img ver-center">
      </div>
    </template>
    <template #extra>
      <n-dropdown :options="dropdownOptions" @select="onDropdownSelected">
        <n-avatar round size="medium" src="/avatar-demo.jpg"></n-avatar>
      </n-dropdown>
    </template>
    <template #subtitle>
      <span id="subtitle">流浪动物救助站</span>
    </template>
  </n-page-header>
  <n-divider style="margin: 0"/>
</template>

<style scoped lang="scss">
.icon-img {
  filter: invert(33%) sepia(82%) saturate(1386%) hue-rotate(350deg) brightness(107%) contrast(91%);
  left: 2.5em;
  width: 4em;
}

.ver-center {
  position: absolute;
  top: 50%;
  transform: translateY(-50%);
}

.header {
  padding: 1em 2em;
}

#header-title {
  width: 2em;
  display: none;
}

#subtitle {
  position: relative;
  left: 8em;
  color: #616161;
}
</style>
