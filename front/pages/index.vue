<script setup>
const { t } = useI18n();
const emit = defineEmits(["showError"]);

function getFormattedDate() {
  const days = [
    "sunday",
    "monday",
    "tuesday",
    "wednesday",
    "thursday",
    "friday",
    "saturday",
  ];
  const months = [
    "january",
    "february",
    "march",
    "april",
    "may",
    "june",
    "july",
    "august",
    "september",
    "october",
    "november",
    "december",
  ];
  const now = new Date();

  return `${t("day." + days[now.getDay()])} ${now.getDate()} ${t("month." + months[now.getMonth()])}`;
}

const query = gql`
  query TopNews {
    news {
      url
      media
      title
      image
      date
    }
  }
`;
const { loading, result, error } = useQuery(query, {});

// If GraphQL API is not working, throw an error to the user.
if (error) {
  emit("showError");
}
</script>

<template>
  <div class="mx-auto max-w-screen-xl px-4 py-2 md:py-16">
    <Accordion
      :question="$t('faq.articles.question')"
      :answer="
        $t('faq.articles.answer', {
          link: '<a href=\'https://github.com/Gravitalia/news\' class=\'text-blue-600 hover:text-blue-800\'>GitHub</a>',
        })
      "
    />

    <div
      class="my-6 w-full h-full bg-zinc-50 dark:bg-zinc-800 border dark:border-zinc-900 rounded-lg shadow-lg px-6 py-5 md:px-20 md:py-8"
    >
      <h2 class="text-lg md:text-2xl font-semibold flex">
        <svg
          xmlns="http://www.w3.org/2000/svg"
          fill="none"
          viewBox="0 0 24 24"
          stroke-width="1.5"
          stroke="currentColor"
          class="size-5 md:size-7 mr-2 mt-1.5 md:mt-1"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            d="M12 9v3.75m9-.75a9 9 0 1 1-18 0 9 9 0 0 1 18 0Zm-9 3.75h.008v.008H12v-.008Z"
          />
        </svg>
        {{ $t("most_important") }}
        <span class="mt-1.5 md:mt-1 ml-2 text-sm md:text-xl font-thin">{{
          $t("of_the", { date: getFormattedDate() })
        }}</span>
      </h2>
      <NuxtLink
        :to="localePath('/summary')"
        class="mt-1 flex hover:underline text-sm text-zinc-500 dark:text-zinc-400 md:w-1/2"
        >{{ $t("read_summary") }}
        <svg
          xmlns="http://www.w3.org/2000/svg"
          fill="none"
          viewBox="0 0 24 24"
          stroke-width="1.5"
          stroke="currentColor"
          class="mt-1 ml-1 size-3"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            d="m8.25 4.5 7.5 7.5-7.5 7.5"
          ></path></svg
      ></NuxtLink>
      <!--<div v-if="error" class="mt-6 flex flex-col items-center">
        <NuxtImg src="/error.svg" width="600" draggable="false" />
        <p class="mt-6 text-md md:text-xl">{{ $t("error.not_my_fault") }}</p>
        <p class="mt-2 text-md">{{ $t("error.internal_server_error") }}</p>
      </div> -->
      <div
        class="mt-6 px-2 md:px-6 lg:px-8 xl:px-0 grid gap-x-4 gap-y-12 md:gap-x-6 lg:gap-x-24 grid-cols-1 md:grid-cols-2 lg:grid-cols-3"
      >
        <CardTopNews :loading="loading || error" :article="result" />
        <CardTopNews :loading="loading || error" :article="result" />
        <CardTopNews :loading="loading || error" :article="result" />
      </div>
    </div>
  </div>
</template>
