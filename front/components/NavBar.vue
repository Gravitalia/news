<script setup>
const { locale, setLocale } = useI18n();
const languageDropdown = ref(false);
</script>

<template>
  <nav>
    <div
      class="mx-auto max-w-screen-xl flex flex-wrap items-center justify-between p-4"
    >
      <NuxtLink :to="localePath('/')" class="flex items-center space-x-3">
        <NuxtImg
          src="/favicon.png"
          format="webp"
          width="40"
          height="40"
          draggable="false"
          alt="Gravitalia News"
        />
        <span
          class="self-center text-2xl font-semibold whitespace-nowrap dark:text-white"
          >Gravitalia <span class="font-thin">News</span></span
        >
      </NuxtLink>
      <div class="flex order-2 rtl:space-x-reverse">
        <div class="flex items-center order-2">
          <button
            type="button"
            class="inline-flex items-center font-medium justify-center px-4 py-2 text-sm text-zinc-800 dark:text-zinc-200 rounded-lg cursor-pointer hover:bg-zinc-100 dark:hover:bg-zinc-700 dark:hover:text-white"
            @click="languageDropdown = !languageDropdown"
          >
            <NuxtImg
              :src="'/flags/' + locale + '.svg'"
              class="w-5 h-5 me-3"
              draggable="false"
            />
            {{ $t("lang." + locale) }}
          </button>
          <div
            v-if="languageDropdown"
            class="absolute mt-44 my-4 text-base list-none bg-white divide-y divide-zinc-100 rounded-lg shadow dark:bg-zinc-700"
          >
            <ul class="py-2 font-medium">
              <li>
                <button
                  class="px-4 py-2 text-sm text-zinc-700 hover:bg-zinc-100 dark:text-zinc-400 dark:hover:bg-zinc-600 dark:hover:text-white"
                  @click.prevent.stop="setLocale('fr')"
                >
                  <div class="inline-flex items-center">
                    <NuxtImg
                      src="/flags/fr.svg"
                      class="w-5 h-5 me-3"
                      draggable="false"
                    />
                    {{ $t("lang.fr") }}
                  </div>
                </button>
              </li>
              <li>
                <button
                  class="px-4 py-2 text-sm text-zinc-700 hover:bg-zinc-100 dark:text-zinc-400 dark:hover:bg-zinc-600 dark:hover:text-white disabled:hover:cursor-not-allowed"
                  @click.prevent.stop="setLocale('us')"
                >
                  <div class="inline-flex items-center">
                    <NuxtImg
                      src="/flags/us.svg"
                      class="w-5 h-5 me-3"
                      draggable="false"
                    />
                    {{ $t("lang.us") }}
                  </div>
                </button>
              </li>
            </ul>
          </div>
        </div>
      </div>
      <div class="flex gap-8">
        <NuxtLink
          :to="localePath('/')"
          class="group transition duration-100 font-semibold flex-1 text-center px-2"
        >
          {{ $t("news") }}
          <span
            class="block max-w-0 group-hover:max-w-full transition-all duration-500 h-0.5 bg-black dark:bg-white"
          ></span>
        </NuxtLink>
        <NuxtLink
          :to="localePath('/summary')"
          class="group transition duration-100 font-semibold flex-1 text-center px-2"
        >
          {{ $t("summary") }}
          <span
            class="block max-w-0 group-hover:max-w-full transition-all duration-500 h-0.5 bg-black dark:bg-white"
          ></span>
        </NuxtLink>
        <NuxtLink
          :to="localePath('/mth')"
          class="group transition duration-100 font-semibold flex-1 text-center px-2"
        >
          {{ $t("quiz") }}
          <span
            class="block max-w-0 group-hover:max-w-full transition-all duration-500 h-0.5 bg-black dark:bg-white"
          ></span>
        </NuxtLink>
        <NuxtLink
          to="mailto:support@gravitalia.com"
          class="group transition duration-100 font-semibold flex-1 text-center px-2"
        >
          {{ $t("help") }}
          <span
            class="block max-w-0 group-hover:max-w-full transition-all duration-500 h-0.5 bg-black dark:bg-white"
          ></span>
        </NuxtLink>
      </div>
    </div>
  </nav>
</template>
