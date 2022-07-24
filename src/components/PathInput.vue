<script setup lang="ts">
import { invoke } from '@tauri-apps/api'
import { ref } from 'vue';

const props = defineProps<{ modelValue: String }>()
const emits = defineEmits(['update:modelValue'])

const valid = ref(false)
const inputPath = ref(props.modelValue)
async function onInputPathChanged(e: any) {
  valid.value = await invoke('is_dir_exist', {path: inputPath.value})
  if (valid.value) { // Check if exist(with rust)
    emits('update:modelValue', inputPath.value)
  }
}
</script>

<template>
  <div class="form-control m-2">
    <label class="label">
      <span v-if="!valid" class="label-text text-red-500">Not a valid path</span>
    </label>
    <div class="input-group flex">
      <input type="text" placeholder="Choose your mods directory" class="input input-bordered flex-1"
        v-model="inputPath" @input="onInputPathChanged"/>
      <button><i class="ri-folder-line"></i></button>
    </div>
  </div>
</template>

<style scoped>
</style>