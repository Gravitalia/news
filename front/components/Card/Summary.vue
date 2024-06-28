<script setup>
defineProps({
  loading: {
    type: Boolean,
    default: false,
    required: false,
  },
  numero: {
    type: Number,
    required: true,
  },
  title: {
    type: String,
    required: true,
  },
  content: {
    type: String,
    required: true,
  },
  learnMore: {
    type: String,
    required: false,
  },
});
</script>

<template>
  <div
    class="my-6 w-full h-full bg-zinc-50 dark:bg-zinc-800 border dark:border-zinc-900 rounded-lg shadow-lg px-6 py-5 md:px-20 md:py-8"
  >
    <h2 class="text-lg md:text-2xl font-semibold flex">
      <div class="size-7 mr-2 mt-1.5 md:mt-1">
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100">
          <circle
            cx="50"
            cy="50"
            r="40"
            class="stroke-black dark:stroke-white"
            stroke-width="3"
            fill="none"
          />
          <text
            x="50%"
            y="50%"
            class="fill-black dark:fill-white"
            dominant-baseline="middle"
            text-anchor="middle"
            font-family="Arial"
            font-size="40"
          >
            {{ numero }}
          </text>
        </svg>
      </div>

      <div
        v-if="loading"
        class="animate-pulse h-4 mt-2 bg-zinc-200 rounded-full dark:bg-zinc-700 w-96"
      ></div>
      <span v-else>{{ title }}</span>
    </h2>
    <Accordion
      v-if="learnMore"
      class="mt-1 flex text-sm text-zinc-500 dark:text-zinc-400"
      :question="$t('read_more_articles')"
      :answer="learnMore"
    />

    <div class="mt-6">
      <div v-if="loading" class="space-y-2.5 animate-pulse max-w-xl">
        <div class="flex items-center w-full">
          <div
            class="h-2.5 bg-zinc-200 rounded-full dark:bg-zinc-700 w-32"
          ></div>
          <div
            class="h-2.5 ms-2 bg-zinc-300 rounded-full dark:bg-zinc-600 w-24"
          ></div>
          <div
            class="h-2.5 ms-2 bg-zinc-300 rounded-full dark:bg-zinc-600 w-full"
          ></div>
        </div>
        <div class="flex items-center w-full max-w-xl">
          <div
            class="h-2.5 bg-zinc-200 rounded-full dark:bg-zinc-700 w-full"
          ></div>
          <div
            class="h-2.5 ms-2 bg-zinc-300 rounded-full dark:bg-zinc-600 w-full"
          ></div>
          <div
            class="h-2.5 ms-2 bg-zinc-300 rounded-full dark:bg-zinc-600 w-full"
          ></div>
        </div>
        <div class="flex items-center w-full max-w-xl">
          <div
            class="h-2.5 bg-zinc-300 rounded-full dark:bg-zinc-600 w-full"
          ></div>
          <div
            class="h-2.5 ms-2 bg-zinc-200 rounded-full dark:bg-zinc-700 w-80"
          ></div>
          <div
            class="h-2.5 ms-2 bg-zinc-300 rounded-full dark:bg-zinc-600 w-full"
          ></div>
        </div>
        <span class="sr-only">{{ $t("loading") }}</span>
      </div>
      <p v-else>{{ content }}</p>
    </div>
  </div>
</template>
