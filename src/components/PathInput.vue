<script setup lang="ts">
import { invoke } from '@tauri-apps/api'
import { open } from '@tauri-apps/api/dialog'
import { ref, watch } from 'vue';

const props = defineProps<{ modelValue: string }>()
const emits = defineEmits(['update:modelValue'])


const valid = ref(true)
const inputPath = ref(props.modelValue)
watch(props, (newValue, oldValue) => {
  inputPath.value = newValue.modelValue
})
async function onInputPathChanged(e: any) {
  valid.value = await invoke('is_dir_exist', {path: inputPath.value})
  if (valid.value) { // Check if exist(with rust)
    emits('update:modelValue', inputPath.value)
  }
}

// function modDirInput(e: any) {
//   console.log(e)
// }

async function onChooseDir(e: any) {
  const select = await open({
    defaultPath: inputPath.value.toString(),
    directory: true,
    multiple: false
  })
  console.log(select)
  if (select != null) {
    valid.value = true;
    inputPath.value = <string>select
    emits('update:modelValue', inputPath.value)
  }
  // let fileInput = document.getElementById('fileInput')
  // fileInput?.click()
}

</script>

<template>
  <div class="form-control m-2">
    <label class="label">
      <span v-if="!valid" class="label-text text-red-500">Not a valid path</span>
    </label>
    <div class="input-group flex">
      <input type="text" placeholder="Choose your mods directory" class="input input-bordered flex-1 focus:"
        v-model="inputPath" @input="onInputPathChanged"/>
        <!-- <input type="file" id="fileInput" @change="modDirInput" hidden webkitdirectory/> -->
      <button @click="onChooseDir"><i class="ri-folder-line"></i></button>
    </div>
  </div>
</template>

<style scoped>
</style>