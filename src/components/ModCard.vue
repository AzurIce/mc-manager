<script setup lang="ts">
import { invoke } from '@tauri-apps/api';
import { join } from '@tauri-apps/api/path';
import { computed } from '@vue/reactivity';

import { onMounted, ref } from 'vue';
import { useStore } from '../store';

const store = useStore()

const modDir = computed(() => store.state.modDir)

const props = defineProps<{
  modFileName: string
}>()

const checked = ref(false)

async function getModInfo() {
  const modPath = await join(modDir.value, props.modFileName)
  console.log(modPath)

  const modInfoJson = <string>await invoke('get_mod_file_info_json', { path: modPath });
  const modInfo = JSON.parse(modInfoJson)
  console.log(modInfo)

  filename.value = modInfo["filename"]
  isFabricMod.value = modInfo["is_fabric_mod"]
  modId.value = modInfo["mod_id"]
  modName.value = modInfo["mod_name"]
  modVersion.value = modInfo["mod_version"]
  gameVersion.value = modInfo["game_version"]
  isBadJsonSyntax.value = modInfo["is_bad_json_syntax"]
}

onMounted(getModInfo)

const filename = ref("")
const isFabricMod = ref(false)
const modId = ref("")
const modName = ref("")
const modVersion = ref("")
const gameVersion = ref("")
const isBadJsonSyntax = ref(false);

</script>

<template>
  <div class="w-full rounded-md border flex p-2 items-center m-1">
    <div class="flex flex-col">
      <div class="flex items-center">
        <h2>{{ modName }}</h2>
        <span class="badge ml-2" v-if="isFabricMod">Fabric</span>
        <span class="badge badge-error ml-2" v-if="isBadJsonSyntax">BadJsonSyntax</span>
      </div>
      <div class="flex">
        <div class="w-16 h-16 bg-gray-200 rounded text-center">
          Icon
        </div>
        <div class="flex flex-col">
          <span class="ml-2">filename: {{ filename }}</span>
          <span class="ml-2">modVersion: {{ modVersion }}</span>
          <span class="ml-2">gameVersion: {{ gameVersion }}</span>
        </div>
      </div>
      <span class="text-gray-400">modId: {{ modId }}</span>
    </div>

    <div class="flex-1"></div>

    <div class="dropdown dropdown-end m-1">
      <div class="indicator">
        <span class="indicator-item badge badge-success"></span>
        <div class="tooltip" data-tip="New Version">
          <label tabindex="0" class="btn">Version</label>
        </div>
      </div>
      <ul tabindex="0" class="dropdown-content menu p-2 shadow bg-base-100 rounded-box w-52">
        <li><a>Item 1</a></li>
        <li><a>Item 2</a></li>
      </ul>
    </div>

    <input type="checkbox" class="checkbox" v-model="checked" />

  </div>
</template>

<style scoped>
</style>