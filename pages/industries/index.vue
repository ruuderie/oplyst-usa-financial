<script setup>
import { ref, computed } from 'vue';

const blogs = ref([
      {
        id: 1,
        title: 'My First Blog',
        content: 'Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed euismod euismod libero, eget suscipit mauris iaculis in.',
        tags: ['tag1', 'tag2', 'tag3']
      },
      {
        id: 2,
        title: 'My Second Blog',
        content: 'Sed at odio id arcu congue lobortis quis non tellus. Donec quis sapien ac arcu aliquet dictum ac eget dolor.',
        tags: ['tag2', 'tag3']
      },
      {
        id: 3,
        title: 'My Third Blog',
        content: 'Aliquam sit amet sapien tincidunt, vestibulum justo eget, lacinia nulla. Sed ac dapibus nisl, nec laoreet augue.',
        tags: ['tag1']
      }
    ]);
const searchTerm = ref('');
const filteredBlogs = computed(() => {
      if (searchTerm.value === '') {
        return blogs.value;
      } else {
        return blogs.value.filter(blog => {
          return blog.tags.some(tag => {
            return tag.toLowerCase().includes(searchTerm.value.toLowerCase());
          });
        });
      }
});

function SetSearchTerm(val) {
  searchTerm.value = val;
}

</script>

<template>

<div class="is-responsive is-mobile">
    <NuxtLayout>
      <section class="hero is-small is-primary is-fluid">
            <div class="hero-body is-fluid">
              <div class="my-3 has-text-centered">

                <p class="title ">
                  Industries We Serve
                </p>
                <p class="subtitle">
                  Gain Valuable Perspectives and Actionable Advice
                </p>
              </div>
            </div>
      </section>
      <section class="container is-fluid">

        <div class="search-container my-5 py-3 is-fluid">
          <label for="title">Search by Tag:</label>
          <input class="input is-rounded" type="text" id="tag-search" v-model="searchTerm" />
        </div>

        <div class="columns tile is-ancestor my-6 py-4 is-fluid">
          <div class="column is-fluid my-4" v-for="blog in filteredBlogs" :key="blog.id">
            <NuxtLink :to="`/insights/${blog.id}`">
              <h2 class="title">{{ blog.title }}</h2>
            </NuxtLink>
            <p class="subtitle">{{ blog.content }}</p>
            <div class="tags">
                <span class="tag my-3 px-4" v-for="tag in blog.tags" :key="tag">
                  <NuxtLink :to="`/insights`" @click="SetSearchTerm(tag)">
                  {{ tag }}
                </NuxtLink>
                </span>

            </div>
          </div>
        </div>
      </section>
      <NuxtPage/>

  </NuxtLayout>

</div>
</template>
