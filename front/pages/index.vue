<script setup>
const { t } = useI18n();

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
</script>

<template>
  <div class="mx-auto max-w-screen-xl px-4 py-16">
    <Accordion
      :question="$t('faq.articles.question')"
      :answer="$t('faq.articles.answer')"
    />

    <div
      class="my-6 w-full h-full bg-zinc-50 dark:bg-dark border dark:border-zinc-900 rounded-lg shadow-lg px-16 py-8"
    >
      <h2 class="text-2xl font-semibold flex">
        <svg
          xmlns="http://www.w3.org/2000/svg"
          fill="none"
          viewBox="0 0 24 24"
          stroke-width="1.5"
          stroke="currentColor"
          class="size-7 mr-2 mt-1"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            d="M12 9v3.75m9-.75a9 9 0 1 1-18 0 9 9 0 0 1 18 0Zm-9 3.75h.008v.008H12v-.008Z"
          />
        </svg>
        {{ $t("most_important") }}
        <span class="mt-1 ml-2 text-xl font-thin">{{
          $t("of_the", { date: getFormattedDate() })
        }}</span>
      </h2>
      <NuxtLink
        :to="localePath('/summary')"
        class="mt-1 flex hover:underline text-sm text-zinc-500 dark:text-zinc-400 w-1/3"
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
      <div v-if="error" class="mt-6 flex flex-col items-center">
        <NuxtImg src="/error.svg" width="600" draggable="false" />
        <p class="mt-6 text-xl">{{ $t("error.not_my_fault") }}</p>
        <p class="mt-2 text-md">{{ $t("error.internal_server_error") }}</p>
      </div>
      <div v-else class="mt-6 grid gap-x-28 gap-y-12 grid-cols-3">
        <CardTopNews :loading="loading" :article="result" />
        <CardTopNews :loading="loading" :article="result" />
        <CardTopNews :loading="loading" :article="result" />
      </div>
    </div>
  </div>
</template>
