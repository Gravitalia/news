<script setup>
defineProps({
  loading: {
    type: Boolean,
    default: false,
    required: false,
  },
  title: {
    type: String,
    required: true,
  },
  url: {
    type: String,
    required: true,
  },
  image: {
    type: String,
    required: false,
  },
  media: {
    type: String,
    required: false,
  },
  date: {
    type: String,
    required: false,
  },
});
</script>

<template>
  <NuxtLink
    :to="article?.url"
    class="w-[300px] hover:underline text-zinc-900 hover:text-black"
    :class="loading ? 'animate-pulse' : ''"
  >
    <NuxtImg
      v-if="!loading && media"
      :src="'/media/' + media + '.png'"
      format="webp"
      width="70"
      :alt="media"
      draggable="false"
    />
    <div
      v-else-if="loading"
      class="h-4 bg-zinc-200 rounded-full dark:bg-zinc-700 w-32"
    ></div>
    <NuxtImg
      v-if="!loading && image"
      :src="image"
      format="webp"
      width="300"
      class="rounded-lg mt-2"
      alt=""
      draggable="false"
    />
    <div
      v-else-if="loading"
      class="flex items-center justify-center w-[300px] h-48 mt-2 bg-zinc-300 rounded-lg dark:bg-zinc-700"
    >
      <svg
        class="size-10 text-zinc-200 dark:text-zinc-600"
        xmlns="http://www.w3.org/2000/svg"
        fill="currentColor"
        viewBox="0 0 16 20"
      >
        <path
          d="M14.066 0H7v5a2 2 0 0 1-2 2H0v11a1.97 1.97 0 0 0 1.934 2h12.132A1.97 1.97 0 0 0 16 18V2a1.97 1.97 0 0 0-1.934-2ZM10.5 6a1.5 1.5 0 1 1 0 2.999A1.5 1.5 0 0 1 10.5 6Zm2.221 10.515a1 1 0 0 1-.858.485h-8a1 1 0 0 1-.9-1.43L5.6 10.039a.978.978 0 0 1 .936-.57 1 1 0 0 1 .9.632l1.181 2.981.541-1a.945.945 0 0 1 .883-.522 1 1 0 0 1 .879.529l1.832 3.438a1 1 0 0 1-.031.988Z"
        />
        <path
          d="M5 5V.13a2.96 2.96 0 0 0-1.293.749L.879 3.707A2.98 2.98 0 0 0 .13 5H5Z"
        />
      </svg>
    </div>
    <p v-if="!loading" class="text-lg">{{ title }}</p>
    <div v-else>
      <div class="h-6 bg-zinc-200 rounded-full dark:bg-zinc-700 mt-2"></div>
      <div
        class="w-2/3 h-6 bg-zinc-200 rounded-full dark:bg-zinc-700 mt-2"
      ></div>
    </div>
    <p v-if="!loading" class="text-sm">Publié il y a XX minutes</p>

    <span v-if="loading" class="sr-only">{{ $t("loading") }}</span>
  </NuxtLink>
</template>
