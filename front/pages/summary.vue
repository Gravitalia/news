<script setup>
const { locale } = useI18n();
const emit = defineEmits(["showError"]);
const query = gql`
  query getNews($country: String!, $limit: Int!) {
    news {
      getNews(country: $country, limit: $limit) {
        title
        summary
        image {
          fullUrl
        }
        source {
          url
          name
        }
        similar {
          source {
            url
            name
          }
        }
      }
    }
  }
`;
const { loading, result, error } = useQuery(query, {
  country: locale,
  limit: 3,
});

// If GraphQL API is not working, throw an error to the user.
if (error) {
  emit("showError");
}
</script>

<template>
  <div class="flex flex-col items-center mx-auto max-w-screen-xl px-4 py-16">
    <CardSummary v-if="loading || error" :loading="true" :numero="1" />
    <CardSummary
      v-for="(news, index) in result.news.getNews"
      v-else
      v-bind:key="news"
      :numero="index + 1"
      :title="news.title"
      :content="news.summary"
      :learn_more="`<a href='${news.source.url}' class='text-blue-600 hover:text-blue-800'>${news.source.name}</a>`"
    />

    <div class="mr-auto">
      <NuxtLink
        :to="localePath('/mcq')"
        class="group transition duration-100 font-semibold flex-1 text-center px-2"
      >
        <div class="flex">
          {{ $t("take_quiz") }}
          <svg
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24"
            stroke-width="1.5"
            stroke="currentColor"
            class="mt-1.5 ml-1 size-3"
          >
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              d="m8.25 4.5 7.5 7.5-7.5 7.5"
            ></path>
          </svg>
        </div>

        <span
          class="block max-w-0 group-hover:max-w-full transition-all duration-500 h-0.5 bg-black dark:bg-white"
        ></span>
      </NuxtLink>
    </div>
  </div>
</template>
