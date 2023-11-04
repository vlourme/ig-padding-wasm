<template>
  <div class="bg-neutral-900 min-h-screen font-mono text-gray-200">
    <main class="px-4 lg:px-0 py-8 max-w-xl mx-auto space-y-4">
      <div>
        <h1 class="text-3xl font-bold text-amber-500">Instagram Padding Tool</h1>
        <p class="text-gray-400">Add borders to your image to match 1:1 (square) format. Useful if you want to post photos
          taken in different aspect
          ratio into a single post.</p>
      </div>

      <div v-if="error" class="bg-red-500 rounded-md p-4">
        <p class="text-red-100">An error occured while processing your images, the format 
          may not be supported?
        </p>
      </div>

      <div v-if="!loading" class="space-y-1">
        <div class="flex justify-between text-sm font-medium">
          <label>Border size</label>
          <label>{{ border }}px</label>
        </div>
        <input v-model.number="border" type="range" min="0" max="200" step="10"
          class="w-full h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer dark:bg-gray-700">
      </div>

      <div v-if="!loading" class="space-y-1">
        <div class="flex justify-between text-sm font-medium">
          <label>Background color</label>
        </div>

        <div class="flex">
          <label class="flex items-center mr-4">
            <input v-model="color" type="radio" value="white" name="color"
              class="w-4 h-4 text-blue-600 mr-2 bg-gray-100 border-gray-300">
            <span>White</span>
          </label>

          <label class="flex items-center mr-4">
            <input v-model="color" type="radio" value="black" name="color"
              class="w-4 h-4 text-blue-600 mr-2 bg-gray-100 border-gray-300">
            <span>Black</span>
          </label>
        </div>
      </div>

      <div v-if="!loading" class="space-y-2">
        <label
          class="flex flex-col items-center justify-center w-full h-32 px-4 transition bg-neutral-800 border border-gray-600 border-dashed rounded-md appearance-none cursor-pointer hover:border-gray-400 focus:outline-none">
          <span class="flex items-center space-x-2">
            <ArrowUpTrayIcon class="w-6 h-6 text-gray-400" />
            <span class="font-medium text-gray-400">
              Drop your images here or
              <span class="text-blue-600 underline">browse</span>
            </span>
          </span>
          <span class="text-gray-500 text-sm">(.jpg, .jpeg and .png only)</span>
          <input type="file" multiple class="hidden" accept=".jpg,.jpeg,.png" @change="onFileChange">
        </label>

        <p class="text-xs text-gray-400 mt-2">Everything is computed locally, no data is sent to a server.</p>
      </div>

      <div v-else class="bg-neutral-800 rounded-md flex flex-col justify-center items-center space-y-2 h-28">
        <ArrowPathIcon class="w-6 h-6 text-gray-400 animate-spin" />
        <p class="text-sm text-gray-400">Computing images...</p>
      </div>

      <div>
        <h2 class="text-2xl font-bold text-amber-500">How this works</h2>
        <p>
          This web application is an experiment using WebAssembly, image processing is done in <a
            href="https://www.rust-lang.org">Rust</a>
          programming language, using <a href="https://rustwasm.github.io/wasm-pack/">wasm-pack</a>.
          Images are computed directly on your browser, nothing is sent to a server.
        </p>
        <p class="mt-2">Source code can be found <a href="https://github.com/vlourme/ig-padding-wasm">here</a>.</p>
      </div>
    </main>
  </div>
</template>

<script setup lang="ts">
import init, { read_image } from 'crate'
import { ArrowPathIcon, ArrowUpTrayIcon } from '@heroicons/vue/24/outline'
import { onMounted, ref } from 'vue';

const border = ref(40);
const color = ref('white');
const loading = ref(false);
const error = ref(false);

onMounted(async () => {
  await init()
})

const onFileChange = async (event: any) => {
  loading.value = true
  error.value = false;

  for (const file of event.target.files as FileList) {
    const buf = await file.arrayBuffer();
    try {
      const result = read_image(new Uint8Array(buf), { borders: border.value, color: color.value })
      const blob = new Blob([result], { type: 'image/jpeg' })
      const url = URL.createObjectURL(blob)
      const link = document.createElement('a')
      link.href = url
      link.download = file.name.replace(/\.[^/.]+$/, "") + '-border.jpg'
      link.click()
    } catch {
      error.value = true
    }
  }

  loading.value = false
}
</script>

