<script setup>
const { locale } = useI18n();
const emit = defineEmits(["showError"]);
const counter = {
  step: ref(0),
  succeed: ref(0),
  failed: ref(0),
  skipped: ref(0),
};
const query = gql`
  query GetQA($country: String!) {
    qa(country: $country) {
      question
      choices
      answer
      article
    }
  }
`;
const { loading, result, error } = useQuery(query, {
  country: locale,
});

// If GraphQL API is not working, throw an error to the user.
if (error) {
  emit("showError");
}
</script>

<template>
  <div class="flex flex-col items-center mx-auto max-w-screen-xl px-4 py-16">
    <Accordion
      :question="$t('faq.no_response.question')"
      :answer="$t('faq.no_response.answer')"
    />

    <Badge class="mt-8 font-mono">
      <div
        v-if="loading || error"
        class="animate-pulse h-3 my-1.5 bg-zinc-200 rounded-full dark:bg-zinc-700 w-12"
      ></div>
      <div v-else>
        {{ counter.step.value + 1 }}
        /
        {{ result?.length || 3 }}
      </div>
    </Badge>

    <div v-if="loading || error">
      <div
        class="mt-8 animate-pulse h-6 mt-2 bg-zinc-200 rounded-full dark:bg-zinc-700 w-96"
      ></div>
      <span class="sr-only">{{ $t("loading") }}</span>
    </div>
    <div
      v-if="counter.step.value < result?.length || 0"
      v-for="(qa, index) in result"
    >
      <div v-if="counter.step.value === index">
        <h2 class="mt-8 font-semibold text-xl">
          {{ qa.question }}
        </h2>

        <div v-for="choice in choices" class="mt-12 grid gap-y-4 grid-cols-1">
          <button
            type="button"
            class="px-40 py-2 font-medium bg-zinc-50 hover:bg-zinc-100 text-zinc-800 dark:bg-zinc-700 dark:hover:bg-zinc-800 dark:text-zinc-300 border shadow-lg rounded-lg"
          >
            {{ choice }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>
