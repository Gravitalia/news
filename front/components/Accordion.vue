<script setup>
defineProps({
  question: {
    type: String,
    required: true,
  },
  answer: {
    type: String,
    required: true,
  },
});
</script>

<template>
  <div class="w-full">
    <details class="group">
      <summary
        class="text-sm flex flex-1 items-center py-4 cursor-pointer font-medium dark:font-normal transition-all hover:underline"
      >
        <svg
          xmlns="http://www.w3.org/2000/svg"
          fill="none"
          viewBox="0 0 24 24"
          stroke-width="1.5"
          stroke="currentColor"
          class="size-4"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            d="M9.879 7.519c1.171-1.025 3.071-1.025 4.242 0 1.172 1.025 1.172 2.687 0 3.712-.203.179-.43.326-.67.442-.745.361-1.45.999-1.45 1.827v.75M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Zm-9 5.25h.008v.008H12v-.008Z"
          />
        </svg>
        <span class="px-2">{{ question }}</span>
      </summary>

      <p
        class="py-1 text-sm text-zinc-500 dark:text-zinc-400 group-open:animate-slide-down"
      >
        {{ answer }}
      </p>
    </details>
  </div>
</template>
